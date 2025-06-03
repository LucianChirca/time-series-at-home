use axum::{
    routing::{post, get},
    Router,
};
use crate::http::endpoints::{handle_post_data, handle_get_data};
use crate::http::ServerConfig;

pub async fn start_server(config: ServerConfig) {
    println!("Starting server on {}", config.url());
    
    let app = Router::new()
        .route("/data", post(handle_post_data))
        .route("/data", get(handle_get_data));

    let listener = tokio::net::TcpListener::bind(config.url()).await.unwrap();
    axum::serve(listener, app).await.unwrap();
} 