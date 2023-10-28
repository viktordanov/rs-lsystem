use thiserror::Error;

use crate::token::TokenId;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum LSystemError {
    #[error("attempted to use unknown token `{0}`")]
    UnknownToken(String),
    #[error("attempted to construct invalid token `{0}`")]
    InvalidToken(String),
    #[error("attempted to construct invalid token ID `{0}` value must be <= 127")]
    InvalidTokenId(TokenId),
    #[error("invalid rule `{0}`")]
    InvalidRule(String),
    #[error("axiom has not been defined")]
    MissingAxiom,
    #[error("io error")]
    IOError(#[from] std::io::Error),
    #[error("there was an unexpected error in another thread")]
    ThreadError,
    #[error("there was an unexpected error: {source}")]
    Other {
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}