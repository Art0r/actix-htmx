use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] diesel::result::Error),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Hashing error: {0}")]
    HashingError(String),
}