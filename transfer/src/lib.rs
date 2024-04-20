pub use serde::{Deserialize, Serialize};

pub type RoomId = u64;

#[derive(Serialize, Deserialize)]
pub struct ConnectRequest {
    pub room: RoomId,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ConnectResponse {
    pub player: bool,
    pub ok: bool,
}

#[derive(Serialize, Deserialize)]
pub struct PlayRequest {
    pub room: RoomId,
    pub player: bool,
    pub board: String,
}

#[derive(Serialize, Deserialize)]
pub struct QueryRequest {
    pub room: RoomId,
    pub player: bool,
}

#[derive(Serialize, Deserialize, Default)]
pub struct QueryResponse {
    pub board: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DisconnectRequest {
    pub room: RoomId,
}
