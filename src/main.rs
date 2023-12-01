use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::{layer::SubscriberExt, Registry};
use zero2prod_antonio::startup::run;
use zero2prod_antonio::{bind_port, get_connection_to_database, LOCAL_HOST_IP};
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    LogTracer::init().expect("Failed to set logger");
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("zero2prod_antonio".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    set_global_default(subscriber).expect("Failed to set subscriber");

    let (connection, server_port) = get_connection_to_database().await;
    let listener = bind_port(format!("{LOCAL_HOST_IP}:{}", server_port));
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
