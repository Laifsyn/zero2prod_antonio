use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;
#[derive(serde::Deserialize)]
#[allow(dead_code)]
pub struct UserEmail {
    name: String,
    email: String,
}

pub async fn subscribe(form: web::Form<UserEmail>, connection: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );
    let _request_span_guard = request_span.enter();
    let query_span = tracing::info_span!("Saving new subscriber details in the database");
    tracing::info!(
        "Request ID {request_id} - captured email:\"{}\", name: \"{}\"",
        form.email,
        form.name
    );
    tracing::info!("Request ID {request_id} - Saving new subscriber details in the database");
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
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!("Request ID {request_id} - New subscriber details have been saved");
            HttpResponse::Ok().finish()
        }
        Err(error) => {
            // #[cfg(debug_assertions)]
            tracing::error!("Request ID {request_id} - Error from SQL: {error:?}\nEnd of Error...",);
            HttpResponse::InternalServerError().finish()
        }
    }
}
