use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
#[derive(serde::Deserialize)]
#[allow(dead_code)]
pub struct UserEmail {
    name: String,
    email: String,
}

pub async fn subscribe(form: web::Form<UserEmail>, connection: web::Data<PgPool>) -> HttpResponse {
    #[cfg(debug_assertions)]
    eprintln!("captured email:\"{}\", name: \"{}\"", form.email, form.name);
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(connection.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(error) => {
            #[cfg(debug_assertions)]
            println!("Error from SQL: {error:?}\nEnd of Error...",);
            HttpResponse::InternalServerError().finish()
        }
    }
}
