const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
    configureWebpack: {
        plugins: [
            new WasmPackPlugin({
                crateDirectory: path.join(__dirname, "wasm-net"),
                outDir: path.join(__dirname, "pkg"),
                outName: "index"
            }),
        ]
    }
}