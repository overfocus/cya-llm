use axum::Router;
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use std::sync::Arc;
use tower_http::cors::{CorsLayer, Any};

mod api;
mod services;

use services::llm::{LlmClient, LlmFactory, LlmClientType};

#[derive(Clone)]
pub struct AppState {
    llm_client: Arc<Box<dyn LlmClient>>,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let llm_client = LlmFactory::create_client(LlmClientType::HuggingFace)
        .expect("Failed to initialize LLM client");

    let app_state = AppState {
        llm_client: Arc::new(llm_client),
    };

    // Set up CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .nest("/api", api::router())
        .layer(cors)
        .fallback_service(ServeDir::new("static"))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}