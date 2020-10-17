use std::io;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to send a frame.")]
    FrameNotSent(#[source] io::Error),

    #[error("Failed to send a query to the database.")]
    QueryNotSent(#[source] Box<dyn std::error::Error>),

    #[error("Failed to connect to the database.")]
    DatabaseConnectionFailed(#[source] io::Error),
}
