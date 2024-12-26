use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let configuration = get_configuration().expect("Failed to read configuration files.");
    let db_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    let address = TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port))
        .expect("Could not bind to port");
    run(address, db_pool)?.await
}
