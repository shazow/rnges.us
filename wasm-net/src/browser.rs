use std::str::FromStr;

use futures::{
    channel::{mpsc, oneshot},
    compat::*,
    future::{ok, poll_fn},
    prelude::*,
};
use wasm_bindgen::prelude::*;

use libp2p_wasm_ext::{ffi, ExtTransport};

pub use console_error_panic_hook::set_once as set_console_error_panic_hook;
pub use console_log::init_with_level as init_console_log;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("hello from rust-wasm");
}

#[wasm_bindgen]
pub struct Client {}

/// Starts the client.
#[wasm_bindgen]
pub async fn start_client(dial: String, log_level: String) -> Result<Client, JsValue> {
    start_inner(dial, log_level)
        .await
        .map_err(|err| JsValue::from_str(&err.to_string()))
}

async fn start_inner(
    dial: String,
    log_level: String,
) -> Result<Client, Box<dyn std::error::Error>> {
    console_error_panic_hook::set_once();
    init_console_log(log::Level::from_str(&log_level)?)?;

    let transport = ExtTransport::new(ffi::websocket_transport());

    // somehow rig an mpsc sender accessible by the browser piped to the service
    // the best way is probably to accept a future::Stream and by default, the service will poll that for any incoming
    // blocks of messages
    // let (rpc_send_tx, mut rpc_send_rx) = mpsc::unbounded::<Message>();
    wasm_bindgen_futures::spawn_local(crate::service(Some(transport), Some(dial)));

    Ok(Client {})
}
