use std::{env, path::PathBuf};

use axum::{
    extract::Query,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use compilation_runner::{CompilationRunner, Compiler};
mod compilation_runner;
use base64::{engine::general_purpose, Engine};
use dotenv::dotenv;
use serde::Deserialize;

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

#[derive(Deserialize, Debug)]
struct QueryParams {
    workspace_root_path: PathBuf,
    target_compilation_path: PathBuf,
    zip_data: String,
}

async fn upload_handler(Query(query_params): Query<QueryParams>) -> impl IntoResponse {
    // Print query params
    println!(
        "Workspace root path: {:?} | Target path: {:?}",
        query_params.workspace_root_path, query_params.target_compilation_path
    );

    let base64_decoded = general_purpose::STANDARD
        .decode(query_params.zip_data)
        .map_err(|e| e.to_string())
        .unwrap();

    if CompilationRunner::prepare_files(base64_decoded)
        .await
        .is_err()
    {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to upload files".to_string(),
        );
    }

    let compilation_runner = CompilationRunner::new(Compiler::Cairo);
    let program_hash = compilation_runner.compile().await.unwrap();

    (StatusCode::OK, hex::encode(program_hash))
}

async fn search_handler() -> impl IntoResponse {
    // TODO: implement search handler (fetch from db and serve the result).
    "search_handler"
}
