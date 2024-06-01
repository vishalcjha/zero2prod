use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use tokio::net::TcpListener;

use crate::{
    greet,
    routes::{health_check, subscribe},
    state::AppState,
};

pub async fn run(listener: TcpListener, connection_pool: PgPool) {
    let state = AppState::new(connection_pool);
    let app = Router::new()
        .route("/", get(greet))
        .route("/health_check", get(health_check))
        .route("/subscribe", post(subscribe))
        .with_state(state);

    axum::serve(listener, app).await.unwrap();
}
