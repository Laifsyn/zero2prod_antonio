use crate::configuration::{get_configuration, Settings};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

pub mod configuration;
pub mod routes;
pub mod startup;
pub mod telemetry;

pub const LOCAL_HOST_IP: &str = "127.0.0.1";

pub fn bind_port(ip_port: String) -> TcpListener {
    TcpListener::bind(ip_port.clone())
        .unwrap_or_else(|error| panic!("Failed to bind random port(in: {ip_port})... {error}"))
    // I didn't use `expect(format!())` because clippy would ask me to rewrite as unwrap_or_else
}

pub async fn get_connection_to_database() -> (PgPool, u16) {
    // Load connection from stored settings
    let configs = get_configuration().expect("Failed to read configuration.");
    let port = configs.application.port;
    (generate_db_pool(configs).await, port)
}
pub async fn generate_db_pool(configs: Settings) -> PgPool {
    let database_name = &configs.database.database_name;
    // Stablish DB pool connection.
    let connection = PgPool::connect_lazy(configs.database.connection_string().expose_secret())
        .unwrap_or_else(|_| panic!("Couldn't connect to Database\n"));
    sqlx::migrate!()
        .run(&connection)
        .await
        .unwrap_or_else(|e| panic!("Couldn't migrate data to {}\nError:{e}", database_name));
    tracing::info!(
        "Established Pool Connection to \"{}\".",
        configs.database.database_name
    );
    connection
}
