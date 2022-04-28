# RadiancePlatform
A simple text-based game framework

# Installation
Head over to [the releases page](https://github.com/Radiance-Platform/RadiancePlatform/releases/) to grab the latest precompiled build of the platform. Download the version for your operating system, and then open the program in your favorite command line terminal. Games can be loaded using the command-line flag like `./radiance --config-path example_game/` or `radiance.exe --config-path example_game/`. The example game will work with any operating system version. 

# Running for Development
First, ensure you have a working Rust installation with cargo. See https://www.rust-lang.org/tools/install

Then, from within the main folder (the one this README is in), run `cargo run -- --config-path example_game/` compile and start the game engine using the provided example game configuration files. Press the escape key to exit.

# Building Releases
Radiance is designed to be run on both Windows and Linux systems. At this time, macOS is not directly supported, but will likely work fine with some customization to these build instructions. Build steps are designed around Ubuntu and will need some slight tweaking to work on other Linux distributions. Directions are based on [this guide](https://stackoverflow.com/questions/31492799/cross-compile-a-rust-application-from-linux-to-windows).

From an Ubuntu Linux installation with Rust and cargo, run the following commands to prepare your environment:
```sh
sudo apt-get install libsdl2-dev mingw-w64 -y
rustup target add x86_64-pc-windows-gnu
rustup toolchain install stable-x86_64-pc-windows-gnu
curl -s https://www.libsdl.org/release/SDL2-devel-2.0.9-mingw.tar.gz | tar xvz -C /tmp
cp -r /tmp/SDL2-2.0.9/x86_64-w64-mingw32/lib/* ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-pc-windows-gnu/lib/
cp /tmp/SDL2-2.0.9/x86_64-w64-mingw32/bin/SDL2.dll .
```

Afterwards, run these two commands to build for Windows and Linux respectively. Build files can be found in the `target` folder, under 
```sh
cargo build --release --target x86_64-pc-windows-gnu
cargo build --release 
```

# Notes / Useful Resources Used
https://doc.rust-lang.org/book/
https://doc.rust-lang.org/rust-by-example/
https://docs.rs/crossterm/latest/crossterm/index.html
https://github.com/crossterm-rs/crossterm/tree/master/examples
https://rustrepo.com/repo/pyros2097-rust-embed-rust-web
https://crates.io/crates/clap
https://github.com/clap-rs/clap/tree/master/examples
https://crates.io/crates/walkdir
https://medium.com/@otukof/build-your-text-editor-with-rust-part-2-74e03daef237

## Yaml/Serde
https://crates.io/crates/yaml-rust
https://docs.rs/yaml-rust/0.4.5/yaml_rust/
https://github.com/chyh1990/yaml-rust

Useful for serializing the YAML data into Rust Structs
https://transform.tools/json-to-rust-serde






