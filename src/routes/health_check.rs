use actix_web::{HttpResponse, Responder};

pub async fn health_check() -> impl Responder {
    #[cfg(debug_assertions)]
    eprintln!("Pong! - Alive");
    HttpResponse::Ok()
}
