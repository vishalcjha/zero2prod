use axum::{extract::State, Form};
use chrono::Utc;
use reqwest::StatusCode;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domain::{SubscriberEmail, SubscriberName},
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct SubscriberInfo {
    pub name: String,
    pub email: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(app_state, subscriber_info),
    fields(request_id = %Uuid::new_v4(),
    subscriber_email = %subscriber_info.email,
    subscriber_name = %subscriber_info.name)
)]
pub async fn subscribe(
    State(app_state): State<AppState>,
    Form(subscriber_info): Form<SubscriberInfo>,
) -> StatusCode {
    let Ok(subscriber_name) = SubscriberName::parse(subscriber_info.name) else {
        return StatusCode::BAD_REQUEST;
    };
    let Ok(subscriber_email) = SubscriberEmail::parse(subscriber_info.email) else {
        return StatusCode::BAD_REQUEST;
    };
    match insert_subscriber(
        app_state.get_db_connection(),
        subscriber_name,
        subscriber_email,
    )
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            println!("Failed to execute query: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

async fn insert_subscriber(
    pool: &PgPool,
    name: SubscriberName,
    email: SubscriberEmail,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r##"
        INSERT INTO subscriptions(id, email, name, subscribed_at)
        VALUES($1, $2, $3, $4)
        "##,
        Uuid::new_v4(),
        email.as_ref(),
        name.as_ref(),
        Utc::now(),
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
