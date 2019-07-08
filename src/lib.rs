//! PlayWithMe rust client

#[macro_use]
extern crate seed;
use protocol::*;
use seed::prelude::*;
use serde::{Deserialize, Serialize};
use web_sys::WebSocket;

mod protocol;
mod view;
mod websocket;

const WS_URL: &str = "wss://tatrix.org/public/games/play-with-me/server";

pub const TOKENS: [&str; 5] = [
    "✗", "❍", "☀",  "☘", "☣",
];

#[wasm_bindgen]
pub fn render() {
    seed::App::build(Model::new(30), update, view::view)
        // `trigger_update_handler` is necessary,
        // because we want to process `seed::update(..)` calls.
        .window_events(|_| vec![trigger_update_handler()])
        .finish()
        .run();

    let search = seed::window()
        .location()
        .search()
        .expect("cannot get location");
    let autoconnect = search == "?autoconnect";
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

    player: Player,

    /// Who is connected
    players: Vec<Player>,

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
            player: Player {
                id: "".into(),
                name: "Anonymous".into(),
                token: Token {
                    code: TOKENS[0].into(),
                    color: "#333".into(),
                },
            },
            players: vec![],
        }
    }

    fn send(&self, message: impl Serialize) {
        let message = serde_json::to_string(&message).expect("cannot create frame");
        self.ws
            .as_ref()
            .expect("sending to a None socket")
            .send_with_str(&message)
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
    TokenColorChange(String),
    Connect,
    Connected,
    ServerMessage(ServerMessage),
    Move { x: u32, y: u32 },
    CleanHistory,
    Nope,
}

fn update(msg: Msg, mut model: &mut Model, _orders: &mut Orders<Msg>) {
    match msg {
        Msg::NameChange(name) => model.player.name = name,
        Msg::SessionChange(session) => model.session = session,
        Msg::TokenChange(code) => model.player.token.code = code,
        Msg::TokenColorChange(color) => {
            log!("Changing color", color);
            model.player.token.color = color
        },
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
                player_name: model.player.name.clone(),
                mode: ConnectionMode::NewGame,
                session: model.session.clone(),
            });
        }
        Msg::Move { x, y } => {
            let cell = Cell {
                coord: Coord { x, y },
                value: model.player.token.clone(),
            };
            model.history.push(cell.clone());
            model.send(ClientMessage::PostMove(cell));
        }
        Msg::CleanHistory => {
            model.send(ClientMessage::CleanHistory);
        }
        Msg::ServerMessage(msg) => match msg {
            ServerMessage::Connected(player) => {
                log!(player, "connected");
                model.players.push(player);
            }
            ServerMessage::Disconnected ( player ) => {
                log!(player, "disconnected");
                model.players.retain(|p| p.id != player.id);
            }
            ServerMessage::Move { cell } => {
                log!(&cell);
                model.history.push(cell);
                // TODO: focus
            }
            ServerMessage::Win { player } => {
                log!(player, "won!");
            }
            ServerMessage::SetCell(cell) => {
                // TODO: check uniqueness
                model.history.push(cell);
            },
            ServerMessage::SetSession {
                session,
                history,
                player,
            } => {
                model.player = player;
                model.session = session;
                model.history = history;

                model.player.token.code = select_token(&model.players);
            }
            ServerMessage::Clean => {
                log!("New game started");
                model.history = History::default();
            }
        },
        Msg::Nope => {}
    }
}

fn select_token(players: &[Player]) -> String {
    TOKENS[players.len() % TOKENS.len()].to_string()
}
