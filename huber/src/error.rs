use thiserror::Error;

#[derive(Error, Debug)]
pub enum HuberError {
    #[error("Config not found: {0:?}")]
    ConfigNotFound(String),

    #[error("Package not found: {0:?}")]
    PackageNotFound(String),

    #[error("Package not installed: {0:?}")]
    PackageNotInstalled(String),

    #[error("Repository already exists: {0:?}")]
    RepoAlreadyExist(String),

    #[error("Repository not found: {0:?}")]
    RepoNotFound(String),

    #[error("Package unable to update")]
    PackageUnableToUpdate(#[source] anyhow::Error),
}
