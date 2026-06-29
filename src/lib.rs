/// Logistics Workflow — Desktop document management for export/shipping.

use std::sync::OnceLock;

pub mod config;
pub mod db;
pub mod bridge;

/// Global tokio runtime, initialized at startup. Used by bridge functions
/// that need to block_on async DB operations from synchronous contexts.
pub static TOKIO_RT: OnceLock<tokio::runtime::Runtime> = OnceLock::new();
