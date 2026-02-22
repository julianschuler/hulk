#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown mode with ID {0}")]
    UnknownMode(usize),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}
