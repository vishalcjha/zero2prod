use tokio::net::TcpListener;
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("0.0.0.0:{}", configuration.application_port);
    let listener = TcpListener::bind(address).await.unwrap();
    let _ = run(listener).await;
}
