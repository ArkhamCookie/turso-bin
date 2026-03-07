//! axum backend

use axum::Json;

use clap::crate_version;

use serde::Serialize;

/// Response for getting the version of pastebin
#[derive(Serialize)]
pub struct VersionResponse {
	pub version: String,
}

/// Give version of pastebin
pub async fn version() -> Json<VersionResponse> {
	let version = crate_version!().to_string();

	Json(VersionResponse {
		version,
	})
}
