use zero2prod_antonio::{bind_port, run, LOCAL_HOST_IP};
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = bind_port(format!("{LOCAL_HOST_IP}:0"));
    println!("Server started: {}", listener.local_addr().unwrap());
    let server = run(listener)?;
    tokio::spawn(async move {
        let _ = server.await;
    });

    Ok(())
}
