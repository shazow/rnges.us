# Rust Deps

```sh
# install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# install wasm target
rustup target add wasm32-unknown-unknown
rustup target add wasm32-unknown-unknown --toolchain nightly

# Build the browser pkg (linked in root/package.json)
wasm-pack build --target bundle -- --features browser

# Build the native service
cargo build --bin bootnode

# Run the local service, take note of the multiaddr
target/debug/bootnode

# Edit the multiaddr in $/views/Home.vue and "yarn serve" the vue project and click woo.  In the native terminal you can now send messages to the browser
# currently, the browser can't send messages, the plumbing isn't done yet.
```