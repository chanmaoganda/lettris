use thiserror::Error;

#[derive(Error, Debug)]
pub enum LettrisError {
    #[error("test error")]
    EnumError,
}

pub type LettrisResult<T> = Result<T, LettrisError>;
