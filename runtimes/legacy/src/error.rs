use klyra_service::loader::LoaderError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Load error: {0}")]
    Load(#[from] LoaderError),
    #[error("Run error: {0}")]
    Run(#[from] klyra_service::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
