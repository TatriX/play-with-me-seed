use serde::{Deserialize, Serialize};

/// Message from the server to the client.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "message", content = "data")]
pub enum ServerMessage {
    Connected(Player),
    Disconnected(Player),
    Move {
        #[serde(rename = "Cell")]
        cell: Cell,
    },
    Win {
        #[serde(rename = "Player")]
        player: String,
    },
    SetCell(Cell),
    SetSession {
        session: String,
        history: History,
        #[serde(rename = "me")]
        player: Player,
    },
    Clean,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub token: Token,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Token {
    pub code: String,
    pub color: String,
}

pub type History = Vec<Cell>;

/// Message from the client to the server.
#[derive(Serialize)]
#[serde(tag = "method", content = "resource")]
pub enum ClientMessage {
    #[serde(rename_all = "camelCase")]
    Connect {
        player_name: String,
        mode: ConnectionMode,
        session: String,
    },
    PostMove(Cell),
    CleanHistory,
}

#[derive(Serialize)]
pub enum ConnectionMode {
    NewGame,
    RandomGame,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cell {
    pub coord: Coord,
    pub value: Token,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coord {
    pub x: u32,
    pub y: u32,
}
