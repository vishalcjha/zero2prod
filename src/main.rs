use tokio::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let _ = run(listener).await;
}