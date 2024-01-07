use zero2prod_antonio::startup::run;
use zero2prod_antonio::telemetry::{get_subscriber, init_subscriber};
use zero2prod_antonio::{bind_port, get_connection_to_database};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod_antonio".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let (connection, server_connection) = get_connection_to_database().await;
    let listener = bind_port(format!(
        "{}:{}",
        server_connection.application.host, server_connection.application.port
    ));
    tracing::info!("Server started: {}", listener.local_addr().unwrap()); // Runtime Logging
    let server = run(listener, connection)?;
    tokio::spawn(async move {
        server
            .await
            .unwrap_or_else(|e| panic!("Unexpected error running the server: {e:?}"));
    })
    .await?;

    Ok(())
}
