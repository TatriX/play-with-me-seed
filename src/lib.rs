//! PlayWithMe rust client
//!
//! Plan:
//! - Draw grid
//! - Connect to a hardcoded session
//!

#[macro_use]
extern crate seed;
use seed::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::{MessageEvent, WebSocket};

const WS_URL: &str = "ws://34.68.64.169:8080";

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
    history: History,
}

impl Model {
    fn new(size: u32) -> Self {
        let ws = WebSocket::new(WS_URL).unwrap();
        register_handlers(&ws);
        Self {
            ws,
            size,
            session: "debug".into(),
            connected: false,
            history: History::default(),
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
enum Msg {
    Connect { player_name: String },
    Connected,
    ServerMessage(ServerMessage),
    Move { x: u32, y: u32 },
}

/// Message from the server to the client.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "message", content = "data")]
enum ServerMessage {
    Connected {
        #[serde(rename = "Player")]
        player_name: String,
    },
    SetHistory {
        #[serde(rename = "History")]
        history: History,
    },
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct History {
    moves: Vec<Cell>,
    players: Vec<String>,
}

/// Message from the client to the server.
#[derive(Serialize)]
#[serde(tag = "method", content = "resource")]
enum ClientMessage {
    Connect {
        #[serde(rename = "Player")]
        player_name: String,
    },
    PostMove {
        #[serde(rename = "Cell")]
        cell: Cell,
    },
    GetHistory,
    CleanHistory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Cell {
    coord: Coord,
    value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Coord {
    row: u32,
    col: u32,
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
            ServerMessage::Connected { player_name } => {
                log!(player_name, "connected");
                model.connected = true;
            }
            ServerMessage::SetHistory { history } => {
                model.history = history;
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

fn register_handlers(ws: &web_sys::WebSocket) {
    register_handler_on_open(ws);
    register_handler_on_message(ws);
    register_handler_on_close(ws);
    register_handler_on_error(ws);
}

// ------ HANDLERS -------

fn register_handler_on_open(ws: &web_sys::WebSocket) {
    let on_open = Closure::wrap(Box::new(move |_| {
        log!("WebSocket connection is open now");
        seed::update(Msg::Connected);
    }) as Box<FnMut(JsValue)>);

    ws.set_onopen(Some(on_open.as_ref().unchecked_ref()));
    on_open.forget();
}

fn register_handler_on_close(ws: &web_sys::WebSocket) {
    let on_close = Closure::wrap(Box::new(|_| {
        log!("WebSocket connection was closed");
    }) as Box<FnMut(JsValue)>);

    ws.set_onclose(Some(on_close.as_ref().unchecked_ref()));
    on_close.forget();
}

fn register_handler_on_message(ws: &web_sys::WebSocket) {
    let on_message = Closure::wrap(Box::new(move |ev: MessageEvent| {
        log!("Client received a message");
        let txt = ev.data().as_string().unwrap();
        let json: ServerMessage = serde_json::from_str(&txt).unwrap();
        log!(&txt);
        seed::update(Msg::ServerMessage(json));
    }) as Box<FnMut(MessageEvent)>);

    ws.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
    on_message.forget();
}

fn register_handler_on_error(ws: &web_sys::WebSocket) {
    let on_error = Closure::wrap(Box::new(|_| {
        log!("Error");
    }) as Box<FnMut(JsValue)>);

    ws.set_onerror(Some(on_error.as_ref().unchecked_ref()));
    on_error.forget();
}
