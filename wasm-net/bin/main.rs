use async_std::task;
use env_logger::{Builder, Env};

fn main() {
    //Result<(), Box<dyn Error>>
    Builder::from_env(Env::default().default_filter_or("info")).init();
    let to_dial = std::env::args().nth(1);
    task::block_on(wasm_net::service(None, to_dial))
}
