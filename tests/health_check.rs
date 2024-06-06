use reqwest::Client;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use tokio::sync::oneshot;
use uuid::Uuid;
use zero2prod::configuration::{DatabaseSettings, Settings};
use zero2prod::routes::SubscriberInfo;
use zero2prod::{configuration::get_configuration, startup::run};
pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgress");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");

    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgress.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate database");

    connection_pool
}
fn spawn_app() -> oneshot::Receiver<(u16, Settings)> {
    let tcp_listener = tokio::net::TcpListener::bind("0.0.0.0:0");
    let (tx, rw) = oneshot::channel();

    let _ = tokio::spawn(async move {
        let tcp_listener = tcp_listener.await.unwrap();
        let mut configuration = get_configuration().expect("Failed to read configuration");
        configuration.database.database_name = Uuid::new_v4().to_string();

        let connection_pool = configure_database(&configuration.database).await;
        let _ = tx.send((tcp_listener.local_addr().unwrap().port(), configuration));

        run(tcp_listener, connection_pool).await;
    });
    rw
}

#[tokio::test]
async fn health_check_works() {
    let config_receiver = spawn_app();
    let (port, _) = config_receiver.await.unwrap();

    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://localhost:{}/health_check", port))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(2), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let config_receiver = spawn_app();
    let (port, config) = config_receiver.await.unwrap();

    let client = Client::new();

    let body = "name=vishal%20kumar&email=vishalcjha%40gmail.com";
    let response = client
        .post(format!("http://localhost:{}/subscribe", port))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("failed to send subscribe message");
    assert_eq!(200, response.status().as_u16());

    let mut connection = PgConnection::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to postgres.");
    let saved = sqlx::query_as!(SubscriberInfo, "SELECT email, name FROM subscriptions")
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription.");
    assert_eq!(saved.email, "vishalcjha@gmail.com");
    assert_eq!(saved.name, "vishal kumar");
}

#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
    let config_receiver = spawn_app();
    let (port, _) = config_receiver.await.unwrap();
    let client = Client::new();

    let data = vec![
        ("name=le%20guin", "missing email"),
        ("email=ursula_le_guin%40gmail.com", "missing name"),
        ("", "missing name and email"),
    ];

    for (body, error_msg) in data {
        let response = client
            .post(format!("http://localhost:{}/subscribe", port))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .unwrap();
        assert_eq!(
            422,
            response.status().as_u16(),
            "Api did not fail with 400 when payload was {}",
            error_msg
        );
    }
}

#[tokio::test]
async fn subscribe_returns_a_200_when_fields_are_present_but_empty() {
    let (port, _) = spawn_app().await.unwrap();
    let client = Client::new();
    let test_cases = vec![
        ("name=&email=vishalcjha%40gmail.com", "empty name"),
        ("name=vishal&email=", "empty email"),
        ("name=vishal&email=definitely-not-an-email", "invalid email"),
    ];

    for (body, description) in test_cases {
        let response = client
            .post(format!("http://localhost:{}/subscribe", port))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .unwrap();
        assert_eq!(
            400,
            response.status().as_u16(),
            "The api did not return a 400 when payload was {}.",
            description
        )
    }
}
