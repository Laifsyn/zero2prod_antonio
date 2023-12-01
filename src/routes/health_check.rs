use actix_web::{HttpResponse, Responder};

pub async fn health_check() -> impl Responder {
    #[cfg(debug_assertions)]
    log::info!("Pong! - Alive");
    HttpResponse::Ok()
}
