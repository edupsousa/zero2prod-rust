use std::net::TcpListener;

use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind port");
    run(listener)?.await
}
