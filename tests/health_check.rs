use std::net::TcpListener;

use rusapi::configuration::{get_configuration, DatabaseSettings};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    // db connection
    let mut configuration = get_configuration().expect("Failed to read config");
    configuration.database.database_name = Uuid::new_v4().to_string();

    let connection_pool = configure_database(&configuration.database).await;

    // let connection_pool = PgPool::connect(&pg_pool)
    //     .await
    //     .expect("Failed connecting to DB");

    let server = rusapi::startup::run(listener, connection_pool.clone())
        .expect("Failed to start bind address");
    let _ = tokio::spawn(server);

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db_pool: connection_pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // create db
    let mut connection = PgConnection::connect(&config.connection_without_db_name())
        .await
        .expect("Failed to connect to db");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");
    // migrate
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to DB");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to do DB migrations");

    connection_pool
}

#[tokio::test]
async fn health_check_works() {
    // the health check is exposed at /health_check;
    // the health check is behind a GET method;
    // the health check always returns a 200;
    // the health check’s response has no body

    let test_app = spawn_app().await;

    let app_address = test_app.address;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health_check", app_address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assertions
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form() {
    let test_app = spawn_app().await;

    let app_address = test_app.address;
    let connection_pool = test_app.db_pool;

    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to send request");

    // write some data to db
    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&connection_pool)
        .await
        .expect("Failed to fetch saved subscription");
    println!("{:?}", saved);
    assert_eq!(200, response.status().as_u16())
}

#[tokio::test]
async fn subscriber_returns_404_for_invalid_form() {
    let test_app = spawn_app().await;

    let app_address = test_app.address;

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    let client = reqwest::Client::new();

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to send form data");
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        )
    }
}
