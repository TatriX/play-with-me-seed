//! PlayWithMe rust client
//!
//! Plan:
//! - Draw grid
//! - Connect to a hardcoded session
//!

#[macro_use]
extern crate seed;
use seed::{prelude::*, App};
use wasm_bindgen::JsCast;
use serde::{Deserialize, Serialize};
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
    size: u32,
    connected: bool,
    ws: WebSocket,
}

impl Model {
    fn new(size: u32) -> Self {
        let ws = WebSocket::new(WS_URL).unwrap();
        register_handlers(&ws);
        Self{
            size,
            connected: false,
            ws,
        }
    }
}



// `Serialize` is required by `seed::update(..)`
// `Deserialize` is required by `trigger_update_handler`
#[derive(Clone, Serialize, Deserialize)]
enum Msg {
    Connect{
        session: String,
        player_name: String,
    },
    Connected,
    ServerMessage(ServerMessage),
    Dummy,
}


/// Message from the server to the client.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ServerMessage {
    pub id: usize,
    pub text: String,
}

/// Message from the client to the server.
#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "tag", content = "contents")]
enum ClientMessage {
    Post {
    },
    Connect {
        tag: &'static str,
        contents: String,
    },
    Get {
    },
    Delete {
    },
}

fn update(msg: Msg, mut model: &mut Model, orders: &mut Orders<Msg>) {
    match msg {
        Msg::Dummy => {
            log!("Dummy!");
        },
        Msg::Connect{session, player_name} => {
            let s = serde_json::to_string(&ClientMessage::Connect{
                tag: "Player",
                contents: player_name,
            }).unwrap();
            model.ws.send_with_str(&format!("[\"{}\", {}]", session, s)).unwrap();
        },
        Msg::Connected => {
            log!("Connected!");

            model.connected = true;
            orders.send_msg(Msg::Connect{
                session: "penis".into(),
                player_name: "Vasya".into(),
            });
        }
        Msg::ServerMessage(msg) => {
            model.connected = true;
            log!("Got {:?}", msg);
        }
    }
}

/// Main view
fn view(model: &Model) -> Vec<El<Msg>> {
    vec![
        h6!["PlayWithMe Rust/Seed edition!", simple_ev(Ev::Click, Msg::Dummy)],
        if model.connected {
            div![draw_grid(model.size)]
        } else {
            div!["Connecting..."]
        }
    ]
}

fn draw_grid(size: u32) -> El<Msg> {
    div![
        class!["grid"],
        (0..size).map(|y| draw_row(size, y)).collect::<Vec<_>>()
    ]
}

fn draw_row(size: u32, y: u32) -> El<Msg> {
    div![
        class!["row"],
        (0..size)
            .map(|x| div![id!(&format!("cell-{}-{}", x, y)), class!["cell"]])
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
        log!("- text message: ", &txt);
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
