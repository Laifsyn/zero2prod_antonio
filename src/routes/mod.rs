mod health_check;
mod subscriptions;

pub use health_check::*;
pub use subscriptions::*;
pub use temp_routes::*;

mod temp_routes {
    use actix_web::{HttpRequest, Responder};
    pub async fn greet(req: HttpRequest) -> impl Responder {
        let name = req.match_info().get("arbitrary_name").unwrap_or("World"); //I forgot how this works...?
        format!("Hello {}!", &name)
    }
}
