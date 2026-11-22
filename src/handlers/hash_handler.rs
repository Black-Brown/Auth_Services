use crate::auth::hash::{hash_password, verify_password};
use axum::response::IntoResponse;

pub async fn test_hash_handler() -> impl IntoResponse {
    let result = tokio::task::spawn_blocking(|| {
        let password = "holamundo123";

        let hash = hash_password(password)?;

        let is_valid = verify_password(password, &hash)?;

        let is_not_valid = verify_password("123", &hash)?;

        Ok::<_, String>((hash, is_valid, is_not_valid))
    })
    .await;

    match result {
        Ok(Ok((hash, is_valid, is_not_valid))) => {
            format!(
                "Password hash: {}\nCorrect: {}\nIncorrect: {}",
                hash, is_valid, is_not_valid
            )
        }
        Ok(Err(e)) => format!("Error: {}", e),
        Err(e) => format!("Task error: {}", e),
    }
}
