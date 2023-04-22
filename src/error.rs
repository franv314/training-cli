pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ApiError(reqwest::Error),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::ApiError(value)
    }
}