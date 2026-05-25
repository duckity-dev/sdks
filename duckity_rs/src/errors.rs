use thiserror::Error;

#[derive(Debug, Error)]
pub enum DuckityError {
    #[error("An error occurred when making a request to Duckity's API.")]
    Request(#[from] reqwest::Error),
}
