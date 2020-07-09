#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("configuration error")]
    Config(#[from] config::ConfigError),
    #[error("(de)serialization error")]
    Serde(#[from] serde_json::Error),
    #[error("could not resolve {0}")]
    MissingFileOrDirectory(&'static str),

    #[error("invalid insertion path: {0}")]
    InvalidPath(std::path::PathBuf),

    #[error("{0} is not in the password store")]
    NotInStore(std::path::PathBuf),

    #[error("rm: cannot remove {0}: Is a directory")]
    IsADirectory(std::path::PathBuf),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
