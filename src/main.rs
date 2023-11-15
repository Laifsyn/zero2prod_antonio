use zero2prod_antonio::startup::run;
use zero2prod_antonio::{bind_port, generate_database_connection, LOCAL_HOST_IP};
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let _connection = generate_database_connection();
    //--database-url "postgres://antonio:12345678@localhost:32768/zero2prod_newsletter"
    let listener = bind_port(format!("{LOCAL_HOST_IP}:0"));
    println!("Server started: {}", listener.local_addr().unwrap());
    let server = run(listener)?;
    let _ = tokio::spawn(async move {
        let _ = server.await;
    })
    .await;

    Ok(())
}
