with import <nixpkgs> {
  overlays = map (uri: import (fetchTarball uri)) [
    https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz
  ];
};

stdenv.mkDerivation {
  name = "rngesus";

  buildInputs = [
    # JS deps
    nodejs
    yarn

    # Rust deps
    wasm-pack
    (latest.rustChannels.nightly.rust.override {
      targets = [ "wasm32-unknown-unknown" ];
    })
  ];

  shellHook = ''
    export PATH="$PWD/node_modules/.bin/:$PATH"
  '';
}
