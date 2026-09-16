use axum::{
    extract::{FromRequestParts, Request},
    http::request::Parts,
    middleware::Next,
    response::Response,
};
use common::models::UserClaims;

use crate::{auth::jwt::verify_token, error::AppError, state::AppState};

pub async fn auth_middleware(
    state: axum::extract::State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = if let Some(auth_header) = request
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|val| val.to_str().ok())
    {
        auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| AppError::Unauthorized("Invalid Authorization header format".to_string()))?
            .to_string()
    } else if let Some(query) = request.uri().query() {
        query
            .split('&')
            .find_map(|pair| {
                let mut parts = pair.split('=');
                if parts.next() == Some("token") {
                    parts.next().map(|s| s.to_string())
                } else {
                    None
                }
            })
            .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".to_string()))?
    } else {
        return Err(AppError::Unauthorized("Missing Authorization header".to_string()));
    };

    let claims = verify_token(&token, &state.jwt_secret)?;
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

/// Extractor to retrieve authenticated UserClaims from request extensions
pub struct CurrentUser(pub UserClaims);

#[axum::async_trait]
impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<UserClaims>()
            .cloned()
            .map(CurrentUser)
            .ok_or_else(|| AppError::Unauthorized("User not authenticated".to_string()))
    }
}
