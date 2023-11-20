use crate::configuration::{get_configuration, Settings};
use sqlx::PgPool;
use std::net::TcpListener;

pub mod configuration;
pub mod routes;
pub mod startup;

pub const LOCAL_HOST_IP: &str = "127.0.0.1";

// We need to mark `run` as public.
// It is no longer a binary entrypoint, therefore we can mark it as async
// without having to use any proc-macro incantation.

pub fn bind_port(ip_port: String) -> TcpListener {
    TcpListener::bind(ip_port.clone())
        .unwrap_or_else(|error| panic!("Failed to bind random port(in: {ip_port})... {error}"))
    // I didn't use `expect(format!())` because clippy would ask me to rewrite as unwrap_or_else
}

pub async fn get_connection_to_database() -> PgPool {
    // Load connection from stored settings
    let configuration: Settings = get_configuration().expect("Failed to read configuration.");
    let connection_string = configuration.database.connection_string();
    // Stablish DB connection.
    let connection = PgPool::connect(&connection_string)
        .await
        .unwrap_or_else(|_| panic!("Couldn't connect to \n{connection_string}\n"));
    //
    sqlx::migrate!()
        .run(&connection)
        .await
        .unwrap_or_else(|e| panic!("Couldn't migrate data to {}\nError:{e}", connection_string));
    println!(
        "Connection to \"{}\" database has been succesful",
        configuration.database.database_name
    );
    connection
}
