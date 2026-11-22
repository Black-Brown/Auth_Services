use axum::response::IntoResponse;
use std::net::SocketAddr;

mod auth;
mod handlers;
mod models;

use handlers::hash_handler::test_hash_handler;

#[tokio::main]

async fn main() {
    let port = 8080;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|e| {
            panic!("Failed to bind to {}: {}", addr, e);
        });

    let router = axum::Router::new()
        .route("/", axum::routing::get(root_handler).post(post_handler))
        .route("/test-password", axum::routing::get(test_hash_handler));

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
