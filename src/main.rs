// src/main.rs
use axum::Router;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

mod api;

#[tokio::main]
async fn main() {
    // Build our application by composing routes
    let app = Router::new()
        // Add API routes under the "/api" path
        .nest("/api", api::router())
        // Serve static files from the "static" directory
        .fallback_service(ServeDir::new("static"));

    // Run our app with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}