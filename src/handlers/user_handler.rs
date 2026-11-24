use axum::{Json, http::StatusCode};

use serde::Serialize;

use crate::extrator::AuthUser;
#[derive(Serialize)]
pub struct UserResponse {
    pub message: String,
    pub user_id: String,
}

pub async fn get_current_user(auth_user: AuthUser) -> Result<Json<UserResponse>, StatusCode> {
    Ok(Json(UserResponse {
        message: "Usuarios Autenticado correctamente".to_string(),
        user_id: auth_user.user_id.to_string(),
    }))
}
