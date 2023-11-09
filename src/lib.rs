use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
pub const LOCAL_HOST: &str = "127.0.0.1:8000";

// We need to mark `run` as public.
// It is no longer a binary entrypoint, therefore we can mark it as async
// without having to use any proc-macro incantation.
pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/", web::get().to(greet))
            .route("/{arbitrary_name}", web::get().to(greet))
    })
    .bind(LOCAL_HOST)?
    .run();
    Ok(server)
}
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("arbitrary_name").unwrap_or("World");
    format!("Hello {}!", &name)
}
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
