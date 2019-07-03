use seed::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::MessageEvent;

use crate::{Msg, ServerMessage};

pub fn register_handlers(ws: &web_sys::WebSocket) {
    register_handler_on_open(ws);
    register_handler_on_message(ws);
    register_handler_on_close(ws);
    register_handler_on_error(ws);
}

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
