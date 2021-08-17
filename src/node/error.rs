use thiserror::Error;

#[derive(Error, Debug)]
pub enum NodeError {
    #[error("invalid argument: `{0}`")]
    InvalidArgument(String),
}
