use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub struct ServerError {
    pub message: String,
}
