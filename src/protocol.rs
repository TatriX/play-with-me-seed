use serde::{Deserialize, Serialize};

/// Message from the server to the client.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "message", content = "data")]
pub enum ServerMessage {
    Connected {
        #[serde(rename = "Player")]
        player: String,
    },
    Disconnected {
        #[serde(rename = "Player")]
        player: String,
    },
    Move {
        #[serde(rename = "Cell")]
        cell: Cell,
    },
    Win {
        #[serde(rename = "Player")]
        player: String,
    },
    SetSession {
        #[serde(rename = "SessionId")]
        session: String,
    },
    SetHistory {
        #[serde(rename = "History")]
        history: History,
    },
    Clean,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct History {
    pub moves: Vec<Cell>,
    pub players: Vec<String>,
}

/// Message from the client to the server.
#[derive(Serialize)]
#[serde(tag = "method", content = "resource")]
pub enum ClientMessage {
    Connect {
        #[serde(rename = "Player")]
        player: String,
    },
    PostMove {
        #[serde(rename = "Cell")]
        cell: Cell,
    },
    GetHistory,
    CleanHistory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cell {
    pub coord: Coord,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coord {
    pub row: u32,
    pub col: u32,
}
