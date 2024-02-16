use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShError {
    #[error("{0}")]
    Sql(String),

    #[error("Unknown")]
    Unknow,
}
