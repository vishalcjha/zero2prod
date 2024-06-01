use sqlx::PgPool;
use tokio::net::TcpListener;
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("0.0.0.0:{}", configuration.application_port);
    let listener = TcpListener::bind(address).await.unwrap();
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to database");

    let _ = run(listener, connection_pool).await;
}
