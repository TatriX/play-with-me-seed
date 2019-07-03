//! PlayWithMe rust client

#[macro_use]
extern crate seed;
use protocol::*;
use seed::prelude::*;
use serde::{Deserialize, Serialize};
use web_sys::WebSocket;

mod protocol;
mod websocket;

const WS_URL: &str = "wss://tatrix.org/public/games/play-with-me/server";

#[wasm_bindgen]
pub fn render() {
    seed::App::build(Model::new(30), update, view)
        // `trigger_update_handler` is necessary,
        // because we want to process `seed::update(..)` calls.
        .window_events(|_| vec![trigger_update_handler()])
        .finish()
        .run();
}

#[derive(Clone)]
struct Model {
    ws: WebSocket,
    size: u32,
    connected: bool,
    session: String,
    players: Vec<String>,
    history: History,
}

impl Model {
    fn new(size: u32) -> Self {
        let ws = WebSocket::new(WS_URL).unwrap();
        websocket::register_handlers(&ws);
        Self {
            ws,
            size,
            session: "debug".into(),
            connected: false,
            history: History::default(),
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
            .send_with_str(&self.frame(message))
            .expect("cannot send message");
    }
}

// `Serialize` is required by `seed::update(..)`
// `Deserialize` is required by `trigger_update_handler`
#[derive(Clone, Serialize, Deserialize)]
pub enum Msg {
    Connect { player_name: String },
    Connected,
    ServerMessage(ServerMessage),
    Move { x: u32, y: u32 },
}

fn update(msg: Msg, mut model: &mut Model, orders: &mut Orders<Msg>) {
    match msg {
        Msg::Move { x, y } => {
            model.send(ClientMessage::PostMove {
                cell: Cell {
                    coord: Coord { row: y, col: x },
                    value: "X".into(),
                },
            });
        }
        Msg::Connect { player_name } => {
            model.send(ClientMessage::Connect { player_name });
            // this method also fetches history, the same way as original client does
            model.send(ClientMessage::GetHistory)
        }
        Msg::Connected => {
            log!("Connected!");

            model.connected = true;
            orders.send_msg(Msg::Connect {
                player_name: "Vasya".into(),
            });
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
                model.history = history;
            }
            ServerMessage::Clean => {
                log!("New game started");
                model.history = History::default();
            }
        },
    }
}

/// Main view
fn view(model: &Model) -> Vec<El<Msg>> {
    vec![
        h6!["PlayWithMe Rust/Seed edition!"],
        if model.connected {
            div![draw_grid(model.size, &model.history)]
        } else {
            div!["Connecting..."]
        },
    ]
}

fn draw_grid(size: u32, history: &History) -> El<Msg> {
    div![
        class!["grid"],
        (0..size)
            .map(|y| draw_row(size, y, history))
            .collect::<Vec<_>>()
    ]
}

fn draw_row(size: u32, y: u32, history: &History) -> El<Msg> {
    div![
        class!["row"],
        (0..size)
            .map(|x| {
                let content = history
                    .moves
                    .iter()
                    .find(|cell| cell.coord.row == y && cell.coord.col == x)
                    .map(|cell| cell.value.clone())
                    .unwrap_or(" ".into());
                div![
                    id!(&format!("cell-{}-{}", x, y)),
                    class!["cell"],
                    simple_ev(Ev::Click, Msg::Move { x, y }),
                    content,
                ]
            })
            .collect::<Vec<_>>()
    ]
}
