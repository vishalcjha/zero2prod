use axum::{
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;

use crate::{greet, routes::health_check, routes::subscribe};

pub async fn run(listener: TcpListener) {
    let app = Router::new()
        .route("/", get(greet))
        .route("/health_check", get(health_check))
        .route("/subscribe", post(subscribe));

    axum::serve(listener, app).await.unwrap();
}
