use axum::{
    routing::post,
    Router,
};
use crate::http::endpoints::handle_post_data;

pub async fn start_server() {
    println!("Starting server on http://localhost:3000");
    
    let app = Router::new()
        .route("/data", post(handle_post_data));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
} 