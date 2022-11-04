//! src/main.rs

use std::net::TcpListener;
use zero_to_rust::configuration::get_configuration;
use zero_to_rust::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");

    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
