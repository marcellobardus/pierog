use axum::{
    body::Body,
    http::{HeaderName, HeaderValue, Response},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum::{extract::Query, http::StatusCode};
use compilation_runner::CompilationRunner;
use std::{env, path::PathBuf};
mod compilation_runner;
use base64::{engine::general_purpose, Engine};
use db::Db;
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Deserialize)]
struct ProgramHashRequest {
    program_hash: String,
}

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

    let program_hash = CompilationRunner::run(
        query_params.workspace_root_path,
        query_params.target_compilation_path,
        &base64_decoded,
    )
    .await
    .unwrap();

    let program_hash_string = hex::encode(program_hash);
    let db = Db::new().unwrap();
    db.set(&program_hash_string, &base64_decoded, "0.13.1")
        .unwrap();

    (StatusCode::OK, program_hash_string)
}

async fn search_handler(
    program_hash: Query<ProgramHashRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO: implement search handler (fetch from db and serve the result).
    let db = match Db::new() {
        Ok(db) => db,
        Err(err) => {
            tracing::error!("Failed to create database connection: {}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    println!("Searching for program: {}", program_hash.program_hash);
    match db.get(&program_hash.program_hash) {
        Ok(db_result) => {
            println!("Found program: {:?}", db_result.version);
            let zip_data = db_result.data;
            let response = Response::builder()
                .header(
                    HeaderName::from_static("content-type"),
                    HeaderValue::from_static("application/zip"),
                )
                .header(
                    HeaderName::from_static("content-disposition"),
                    HeaderValue::from_str(&format!(
                        "attachment; filename=\"{}.zip\"",
                        program_hash.program_hash
                    ))
                    .unwrap(),
                )
                .body(Body::from(zip_data))
                .unwrap();
            Ok(response)
        }
        Err(err) => {
            tracing::error!("Failed to fetch program: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
