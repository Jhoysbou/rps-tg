use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub struct RoomError {
    pub message: String,
}
