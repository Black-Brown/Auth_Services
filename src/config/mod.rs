use std::sync::Arc;

#[derive(Clone)]
pub struct AuthConfig {
    pub jwt_secret: Arc<str>,
}
