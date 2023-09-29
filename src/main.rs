use env_logger::Env;
use std::net::TcpListener;

use rusapi::{configuration::get_configuration, startup};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let configuration = get_configuration().expect("Failed to read configuration file");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed binding to port specified");

    let pg_connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to postgres");

    startup::run(listener, pg_connection)?.await
}
