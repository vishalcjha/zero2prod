use axum::{extract::State, Form};
use chrono::Utc;
use reqwest::StatusCode;
use serde::Deserialize;
use uuid::Uuid;

use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct SubscriberInfo {
    pub name: String,
    pub email: String,
}

pub async fn subscribe(
    State(app_state): State<AppState>,
    Form(subscriber_info): Form<SubscriberInfo>,
) -> StatusCode {
    match sqlx::query!(
        r##"
        INSERT INTO subscriptions(id, email, name, subscribed_at)
        VALUES($1, $2, $3, $4)
        "##,
        Uuid::new_v4(),
        subscriber_info.email,
        subscriber_info.name,
        Utc::now(),
    )
    .execute(app_state.get_db_connection())
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            println!("Failed to execute query: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
