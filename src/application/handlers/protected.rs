use axum::Json;
use serde::Serialize;
use crate::domain::models::user::Claims;

#[derive(Serialize)]
pub struct ProtectedResponse {
    message: String,
    user_id: i32,
}

pub async fn handler(
    claims: axum::extract::Extension<Claims>,
) -> Json<ProtectedResponse> {
    Json(ProtectedResponse {
        message: "This is a protected endpoint".to_string(),
        user_id: claims.sub,
    })
} 