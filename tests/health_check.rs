use sqlx::{Connection, PgConnection, PgPool};
use std::net::TcpListener;

use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::test]
async fn health_check_test() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute req");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let configuration = get_configuration().expect("failed to read configuration files");
    let conn_string = configuration.database.connection_string();

    let mut conn = PgConnection::connect(&conn_string)
        .await
        .expect("Failed to connect to Postgres");

    let body = "name=lu%20ana&email=luana%40gmail.com";

    let resp = client
        .post(format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute req");

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&mut conn)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "luana@gmail.com");
    assert_eq!(saved.name, "lu ana");

    assert_eq!(200, resp.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_400_for_invalid_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le", "missing email"),
        ("email=le%40gmail.com", "missing name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_msg) in test_cases {
        let resp = client
            .post(format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute req");

        assert_eq!(
            400,
            resp.status().as_u16(),
            "API did not fail with 400 Bad Request when payload was {}",
            error_msg
        );
    }
}

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a random port");
    let port = listener.local_addr().unwrap().port();

    let address = format!("http://127.0.0.1:{}", port);
    let config = get_configuration().expect("Failed to read configuration files");
    let conn_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let server = run(listener, conn_pool.clone()).expect("Failed to bind address");

    #[allow(clippy::let_underscore_future)]
    let _ = tokio::spawn(server);

    TestApp {
        address,
        db_pool: conn_pool,
    }
}
