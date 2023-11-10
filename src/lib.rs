use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
pub const LOCAL_HOST_IP: &str = "127.0.0.1";

// We need to mark `run` as public.
// It is no longer a binary entrypoint, therefore we can mark it as async
// without having to use any proc-macro incantation.
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .route("/{arbitrary_name}", web::get().to(greet))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
pub fn bind_port(ip_port: String) -> TcpListener {
    TcpListener::bind(ip_port.clone())
        .unwrap_or_else(|error| panic!("Failed to bind random port(in: {ip_port})... {error}"))
    // I didn't use `expect(format!())` because clippy would ask me to rewrite as unwrap_or_else
}
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("arbitrary_name").unwrap_or("World");
    format!("Hello {}!", &name)
}
async fn subscribe(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
