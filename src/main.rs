use axum::{Extension, Router, response::IntoResponse, routing::get};
use std::{net::SocketAddr, sync::Arc};

mod auth;
mod config;
mod extrator;
mod handlers;
mod models;

use config::AuthConfig;
use handlers::hash_handler::test_hash_handler;
use handlers::user_handler::get_current_user;

#[tokio::main]

async fn main() {
    dotenvy::dotenv().ok();

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET no esta configurado");

    let auth_config = AuthConfig {
        jwt_secret: Arc::from(secret),
    };

    let port = 8080;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|e| {
            panic!("Failed to bind to {}: {}", addr, e);
        });

    let router = Router::new()
        .route("/", get(root_handler).post(post_handler))
        .route("/users/me", get(get_current_user))
        .route("/test-password", get(test_hash_handler))
        .layer(Extension(auth_config));

    println!("listener on {}", listener.local_addr().unwrap());

    axum::serve(listener, router)
        .await
        .expect("Error ejecutando el servidor")
}

async fn root_handler() -> impl IntoResponse {
    println!("Procesing root handler");

    String::from("Auth Services")
}

async fn post_handler() -> impl IntoResponse {
    println!("Procesing Post handler");

    String::from("POST: Auth Services")
}
