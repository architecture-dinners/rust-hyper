# rust-hyper
Getting Hyped Up on Rust

## Installing Rust
1. Get yourself a `rustup` [here](curl https://sh.rustup.rs -sSf | sh) and install the latest version of `stable`.
2. Install `rustfmt` so that we can all (programmatically) agree on what _good code_ looks like.

## Running the Server Locally
To ensure that everything compiles you can run `cargo check`. To build some binaries you can run `cargo build`. To run the server via `cargo` you can run `cargo run`. 

### Server Configuration Variables
- `RUST_LOG = <logger config>` (see docs [here](https://github.com/sebasmagri/env_logger/))
  - A good default value here might be `RUST_LOG=rust_hyper=info`
- `NOM_SERVER_HOST = <hostname>`
- `NOM_SERVER_PORT = <port>`

## Building with Docker
To build with docker, just run `docker-compose up`. Please note that this'll take quite a while and consume a fair bit of disk space.
