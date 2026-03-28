use std::path::PathBuf;
use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum MarkError {
    #[error("Input file not found: {0}")]
    InputNotFound(PathBuf),

    #[error("Cannot determine home directory")]
    NoHomeDir,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Config error: {0}")]
    Config(String),
}
