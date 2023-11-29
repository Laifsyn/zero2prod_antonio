use env_logger::Env;
use zero2prod_antonio::startup::run;
use zero2prod_antonio::{bind_port, configuration, get_connection_to_database, LOCAL_HOST_IP};
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let connection = get_connection_to_database();
    let server_port = configuration::get_configuration().unwrap().application_port;
    let listener = bind_port(format!("{LOCAL_HOST_IP}:{}", server_port));
    println!("Server started: {}", listener.local_addr().unwrap()); // Runtime Logging
    let server = run(listener, connection.await)?;
    let _ = tokio::spawn(async move {
        let _ = server.await;
    })
    .await;

    Ok(())
}
