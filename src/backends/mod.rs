//! Different web backend for pastebin

#[cfg(feature = "axum")]
/// axum backend
pub mod axum;

#[cfg(feature = "hyper")]
/// hyper backend
pub mod hyper;
