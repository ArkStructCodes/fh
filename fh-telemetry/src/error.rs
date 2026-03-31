use std::io;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Connection(#[from] io::Error),
    #[error("unsupported packet format")]
    Unsupported,
    #[error("unexpected internal error")]
    Internal,
}

pub type Result<T> = std::result::Result<T, Error>;
