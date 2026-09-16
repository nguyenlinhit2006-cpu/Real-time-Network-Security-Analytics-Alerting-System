use common::models::UserRole;

use crate::{auth::middleware::CurrentUser, error::AppError};

pub fn require_role(user: &CurrentUser, allowed_roles: &[UserRole]) -> Result<(), AppError> {
    if allowed_roles.contains(&user.0.role) {
        Ok(())
    } else {
        Err(AppError::Forbidden(format!(
            "Role '{:?}' is not authorized to access this resource",
            user.0.role
        )))
    }
}

pub fn require_admin(user: &CurrentUser) -> Result<(), AppError> {
    require_role(user, &[UserRole::Admin])
}

pub fn require_analyst_or_admin(user: &CurrentUser) -> Result<(), AppError> {
    require_role(user, &[UserRole::Admin, UserRole::Analyst])
}
