use std::{net, str::FromStr};

use config::{Config, ConfigError, Environment};

pub fn hyper_server_addr() -> Result<net::SocketAddr, net::AddrParseError> {
    net::SocketAddr::from_str(&format!(
        "{host}:{port}",
        host = CONFIG.server.host,
        port = CONFIG.server.port
    ))
}

lazy_static! {
    static ref CONFIG: Configuration = init_config().unwrap();
}

#[derive(Deserialize)]
struct Configuration {
    server: ServerConfig,
}

#[derive(Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
}

fn init_config() -> Result<Configuration, ConfigError> {
    let mut config = Config::new();
    config.set_default("server.host", "127.0.0.1")?;
    config.set_default("server.port", 8085)?;

    config.merge(Environment::new().prefix("nom").separator("_"))?;
    config.try_into()
}
