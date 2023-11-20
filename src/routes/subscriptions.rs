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
    let _r = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    // We use `get_ref` to get an immutable reference to the `PgPool`
    // wrapped by `web::Data`.
    .execute(connection.get_ref())
    .await;
    // println!("Error from SQL: {_r:#?}\nEnd of Error...");
    // if let Err(error) = _r {
    //     #[cfg(debug_assertions)]
    //     println!("Error from SQL: {error:?}\nEnd of Error...",);
    //     return HttpResponse::error(&error);
    // }
    HttpResponse::Ok().finish()
}
