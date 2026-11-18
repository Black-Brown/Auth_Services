use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,

    #[serde(skip_serializing)]
    pub password_hash: String,
    pub create_at: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub username: Option<String>,
    pub new_password: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub message: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionClaim {
    pub user_id: String,
    pub exp: usize,
}
