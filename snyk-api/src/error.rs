use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Reqwest: {0}")]
    Reqwest(String),

    #[error("Failed to parse response data")]
    ParseError,

    #[error("Missing SNYK_API_KEY environment variable")]
    EnvironmentError,
}

impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(err.to_string())
    }
}
