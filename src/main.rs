use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("arbitrary_name").unwrap_or("World");
    format!("Hello {}!", &name)
}
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Server start....");

    HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/", web::get().to(greet))
            .route("/{arbitrary_name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
