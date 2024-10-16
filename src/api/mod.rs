use axum::{
    routing::get,
    Router,
    response::Html,
};

mod story;

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/hello", get(hello))
        .route("/greet/:name", get(greet))
        .nest("/story", story::router())
}

async fn hello() -> Html<String> {
    Html("<p>Hello from Rust!</p>".to_string())
}

async fn greet(axum::extract::Path(name): axum::extract::Path<String>) -> Html<String> {
    Html(format!("<p>Hello, {}!</p>", name))
}