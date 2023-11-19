use zero2prod_antonio::startup::run;
use zero2prod_antonio::{bind_port, configuration, generate_database_connection, LOCAL_HOST_IP};
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let connection = generate_database_connection();
    //--database-url "postgres://antonio:12345678@localhost:32768/zero2prod_newsletter"
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
