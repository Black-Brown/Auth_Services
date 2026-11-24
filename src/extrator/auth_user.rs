use axum::{
    Extension,
    extract::FromRequestParts,
    http::{StatusCode, header::AUTHORIZATION, request::Parts},
    response::{IntoResponse, Response},
};

use uuid::Uuid;

use crate::{auth::token::verify_token, config::AuthConfig};

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let config = parts
            .extensions
            .get::<Extension<AuthConfig>>()
            .ok_or_else(|| StatusCode::INTERNAL_SERVER_ERROR.into_response())?
            .0
            .clone();

        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .ok_or_else(|| StatusCode::UNAUTHORIZED.into_response())?;

        let auth_header = auth_header
            .to_str()
            .map_err(|_| StatusCode::UNAUTHORIZED.into_response())?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(StatusCode::UNAUTHORIZED.into_response())?;

        let claims = verify_token(token, &config.jwt_secret)
            .map_err(|_| StatusCode::UNAUTHORIZED.into_response())?;

        let user_id =
            Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::UNAUTHORIZED.into_response())?;

        Ok(AuthUser { user_id })
    }
}
