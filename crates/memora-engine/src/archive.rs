//! Local archive module stub (cloud archive disabled for Memora local-first architecture).

use axum::{extract::State, http::StatusCode, response::Json as JsonResponse};
use serde_json::{json, Value};
use crate::server::AppState;

pub async fn archive_init(State(_state): State<AppState>) -> (StatusCode, JsonResponse<Value>) {
    (StatusCode::NOT_IMPLEMENTED, JsonResponse(json!({"error": "Cloud archive disabled in Memora local-first mode"})))
}

pub async fn archive_configure(State(_state): State<AppState>) -> (StatusCode, JsonResponse<Value>) {
    (StatusCode::NOT_IMPLEMENTED, JsonResponse(json!({"error": "Cloud archive disabled in Memora local-first mode"})))
}

pub async fn archive_status(State(_state): State<AppState>) -> (StatusCode, JsonResponse<Value>) {
    (StatusCode::OK, JsonResponse(json!({"status": "disabled"})))
}

pub async fn archive_run(State(_state): State<AppState>) -> (StatusCode, JsonResponse<Value>) {
    (StatusCode::NOT_IMPLEMENTED, JsonResponse(json!({"error": "Cloud archive disabled in Memora local-first mode"})))
}

pub async fn archive_download(State(_state): State<AppState>) -> (StatusCode, JsonResponse<Value>) {
    (StatusCode::NOT_IMPLEMENTED, JsonResponse(json!({"error": "Cloud archive disabled in Memora local-first mode"})))
}
