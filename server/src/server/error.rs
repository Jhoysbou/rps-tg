use derive_more::{Display, Error};

use crate::room::error::RoomError;

#[derive(Debug, Display, Error)]
pub struct ServerError {
    pub message: String,
}

impl From<RoomError> for ServerError {
    fn from(value: RoomError) -> Self {
        Self {
            message: value.message,
        }
    }
}
