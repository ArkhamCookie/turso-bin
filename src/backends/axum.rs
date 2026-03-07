//! axum backend

use crate::pastes::{Paste, Pastes};

use std::sync::Arc;

use axum::Json;
use axum::extract::{Path, State};
use axum::response::IntoResponse;

use clap::crate_version;

use serde::Serialize;

use turso::Connection;

use tokio::sync::RwLock;

/// App state for axum
#[derive(Debug)]
pub struct AppState {
	pub connection: Connection,
}
/// Shared state of data for axum
pub type SharedState = Arc<RwLock<AppState>>;

/// Response for getting the version of pastebin
#[derive(Serialize)]
pub struct VersionResponse {
	pub version: String,
}

/// Get paste by id
pub async fn get_paste_by_id(
	Path(id): Path<u64>,
	State(state): State<SharedState>,
) -> impl IntoResponse {
	let state = state.read().await;
	let connection = state.connection.clone();

	Json(Paste::get_by_id(&connection, id).await.unwrap())
}

/// Get paste by link
pub async fn get_paste_by_link(
	Path(link): Path<String>,
	State(state): State<SharedState>,
) -> impl IntoResponse {
	let state = state.read().await;
	let connection = state.connection.clone();

	Json(Paste::get_by_link(&connection, link).await.unwrap())
}

/// Get all pastes from database using axum's state and return in JSON response
pub async fn get_pastes(State(state): State<SharedState>) -> impl IntoResponse {
	let state = state.read().await;

	let connection = state.connection.clone();

	Json(Pastes::fetch(&connection).await.unwrap())
}

/// Give version of pastebin
pub async fn version() -> Json<VersionResponse> {
	let version = crate_version!().to_string();

	Json(VersionResponse { version })
}
