use thiserror::Error;

#[derive(Debug, Error)]
pub enum DoError {
    #[error("Incorrect login details")]
    InvalidLogin
}
