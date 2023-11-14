use actix_web::{web, HttpResponse};
pub async fn subscribe(_form: web::Form<UserEmail>) -> HttpResponse {
    #[cfg(debug_assertions)]
    eprintln!("email:{}\nname: {}\n", _form.email, _form.name);
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
#[allow(dead_code)]
pub struct UserEmail {
    name: String,
    email: String,
}
