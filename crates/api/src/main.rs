use std::env;

use axum::{
    extract::Multipart,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use compilation_runner::{CompilationRunner, Compiler};
mod compilation_runner;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route(
            "/upload",
            post(upload_handler).layer(axum::extract::DefaultBodyLimit::disable()),
        )
        .route("/search", get(search_handler));

    let port = env::var("PORT").unwrap_or("3000".to_string());
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", "0.0.0.0", port))
        .await
        .unwrap();

    println!("Listening on port {}", port);

    axum::serve(listener, app).await.unwrap();
}

async fn upload_handler(files: Multipart) -> impl IntoResponse {
    if CompilationRunner::prepare_files(files).await.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to upload files".to_string(),
        );
    }

    let compilation_runner = CompilationRunner::new(Compiler::Cairo);
    match compilation_runner.compile().await {
        Ok(output) => (StatusCode::OK, output),
        Err(err_msg) => {
            tracing::error!("Compilation failed: {}", err_msg);
            (StatusCode::INTERNAL_SERVER_ERROR, err_msg)
        }
    }
}

async fn search_handler() -> impl IntoResponse {
    // TODO: implement search handler (fetch from db and serve the result).
    "search_handler"
}
