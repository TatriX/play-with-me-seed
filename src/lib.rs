//! PlayWithMe rust client

#[macro_use]
extern crate seed;
use protocol::*;
use seed::prelude::*;
use serde::{Deserialize, Serialize};
use web_sys::WebSocket;

mod protocol;
mod websocket;
mod view;

const WS_URL: &str = "wss://tatrix.org/public/games/play-with-me/server";

pub const TOKENS: [&str; 4] = ["☀", "☉", "☘", "☣"];

#[wasm_bindgen]
pub fn render() {
    seed::App::build(Model::new(30), update, view::view)
        // `trigger_update_handler` is necessary,
        // because we want to process `seed::update(..)` calls.
        .window_events(|_| vec![trigger_update_handler()])
        .finish()
        .run();

    let autoconnect = true;
    if autoconnect {
        seed::update(Msg::Connect);
    }
}

#[derive(Clone)]
pub struct Model {
    /// Well, it's a websocket!
    ws: Option<WebSocket>,

    /// Size of the field
    size: u32,

    /// Current game stage
    stage: Stage,

    /// Current session id
    session: String,

    /// Current token ("x" or "o" for now)
    token: String,

    /// Current player name
    player: String,

    /// Who is connected
    players: Vec<String>,

    /// State of the game field
    history: History,
}

#[derive(Clone)]
pub enum Stage {
    Lobby,
    Loading,
    Gameplay,
}

impl Model {
    fn new(size: u32) -> Self {
        Self {
            size,
            ws: None,
            session: "global".into(),
            stage: Stage::Lobby,
            history: History::default(),
            player: "Anonymous".into(),
            token: TOKENS[0].into(),
            players: vec![],
        }
    }

    /// Create frame ready to be sent over the websocket with the
    /// current session id.
    fn frame(&self, message: impl Serialize) -> String {
        let frame = (&self.session, message);
        serde_json::to_string(&frame).expect("cannot create frame")
    }

    fn send(&self, message: impl Serialize) {
        self.ws
            .as_ref()
            .expect("sending to a None socket")
            .send_with_str(&self.frame(message))
            .expect("cannot send message");
    }
}

// `Serialize` is required by `seed::update(..)`
// `Deserialize` is required by `trigger_update_handler`
#[derive(Clone, Serialize, Deserialize)]
pub enum Msg {
    NameChange(String),
    SessionChange(String),
    TokenChange(String),
    Connect,
    Connected,
    ServerMessage(ServerMessage),
    Move { x: u32, y: u32 },
    CleanHistory,
    Nope,
}

fn update(msg: Msg, mut model: &mut Model, _orders: &mut Orders<Msg>) {
    match msg {
        Msg::NameChange(player) => model.player = player,
        Msg::SessionChange(session) => model.session = session,
        Msg::TokenChange(token) => model.token = token,
        Msg::Connect => {
            let ws = WebSocket::new(WS_URL).expect("websocket failure");
            websocket::register_handlers(&ws);
            model.ws = Some(ws);
            model.stage = Stage::Loading;
        }
        Msg::Connected => {
            log!("Connected!");
            model.stage = Stage::Gameplay;
            model.send(ClientMessage::Connect {
                player: model.player.clone(),
            });
            model.send(ClientMessage::GetHistory);
        }
        Msg::Move { x, y } => {
            model.send(ClientMessage::PostMove {
                cell: Cell {
                    coord: Coord { row: y, col: x },
                    value: model.token.clone(),
                },
            });
        }
        Msg::CleanHistory => {
            model.send(ClientMessage::CleanHistory);
        }
        Msg::ServerMessage(msg) => match msg {
            ServerMessage::Connected { player } => {
                log!(player, "connected");
                model.players.push(player);
            }
            ServerMessage::Disconnected { player } => {
                log!(player, "disconnected");
                model.players.retain(|name| name != &player);
            }
            ServerMessage::Move { cell } => {
                log!(&cell);
                model.history.moves.push(cell);
                // TODO: focus
            }
            ServerMessage::Win { player } => {
                log!(player, "won!");
            }
            ServerMessage::SetSession { session } => {
                model.session = session;
            }
            ServerMessage::SetHistory { history } => {
                model.token = select_token(&history);
                model.history = history;
            }
            ServerMessage::Clean => {
                log!("New game started");
                model.history = History::default();
            }
        },
        Msg::Nope => {}
    }
}

fn select_token(history: &History) -> String {
    TOKENS[history.players.len() % TOKENS.len()].to_string()
}
