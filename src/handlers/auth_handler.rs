use serde::{Deserialize, Serialize};

use crate::extrator::AuthUser;

#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
    pub user_id: String,
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

pub async fn login_handler() {
    // Aquí implementaremos el login.
}
