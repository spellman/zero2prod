use std::net;
use zero2prod::configuration;
use zero2prod::startup;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let configuration = configuration::get_configuration().expect("Failed to read configuration.");
    let address = &format!("127.0.0.1:{}", configuration.application_port);
    let listener = net::TcpListener::bind(address)?;
    startup::run(listener)?.await
}
