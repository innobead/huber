use thiserror::Error;

#[derive(Error, Debug)]
pub enum HuberError {
    #[error("config not found: {0:?}")]
    ConfigNotFound(String),
}
