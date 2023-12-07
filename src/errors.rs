pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Http(#[from] reqwest::Error),
    #[error("{}", .0.message)]
    Service(crate::model::Error),
    #[error("Invalid domain: {0}")]
    Domain(#[from] std::num::ParseIntError),
}
