use zero2prod_antonio::run;
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Server start....");
    run().await?.await
}
