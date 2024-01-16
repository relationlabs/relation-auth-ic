use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpError {
    #[error("Failed to parse uri: {0:?}")]
    Uri(#[from] fluent_uri::ParseError),

    #[error("Failed to parse body as JSON: {0:?}")]
    Json(#[from] serde_json::Error),
}
