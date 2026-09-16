use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use tracing::info;

use crate::state::AppState;

pub async fn ws_alerts_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_alerts_socket(socket, state))
}

async fn handle_alerts_socket(mut socket: WebSocket, state: AppState) {
    let mut rx = state.alert_broadcast.subscribe();
    info!("Client connected to /ws/alerts stream");

    while let Ok(alert) = rx.recv().await {
        if let Ok(json) = serde_json::to_string(&alert) {
            if socket.send(Message::Text(json)).await.is_err() {
                // Client disconnected
                break;
            }
        }
    }

    info!("Client disconnected from /ws/alerts stream");
}

pub async fn ws_traffic_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_traffic_socket(socket, state))
}

async fn handle_traffic_socket(mut socket: WebSocket, state: AppState) {
    let mut rx = state.traffic_broadcast.subscribe();
    info!("Client connected to /ws/traffic live feed");

    while let Ok(event) = rx.recv().await {
        if let Ok(json) = serde_json::to_string(&event) {
            if socket.send(Message::Text(json)).await.is_err() {
                break;
            }
        }
    }

    info!("Client disconnected from /ws/traffic live feed");
}
