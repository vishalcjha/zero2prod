use axum::{
    extract::MatchedPath,
    http::Request,
    routing::{get, post},
    Router,
};
use once_cell::sync::Lazy;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info_span;
use tracing_subscriber::util::SubscriberInitExt;

use crate::{
    greet,
    routes::{health_check, subscribe},
    state::AppState,
    telemetry::get_subscriber,
};

static TRACING: Lazy<()> = Lazy::new(|| {
    get_subscriber("zero2prod".to_owned(), "info".into()).init();
});

pub async fn run(listener: TcpListener, connection_pool: PgPool) {
    let state = AppState::new(connection_pool);
    Lazy::force(&TRACING);
    let app = Router::new()
        .route("/", get(greet))
        .route("/health_check", get(health_check))
        .route("/subscribe", post(subscribe))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);
                info_span!("http_request", method = ?request.method(), matched_path)
            }),
        )
        .with_state(state);

    axum::serve(listener, app).await.unwrap();
}
