use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{NewSubscriber, SubscriberName};
#[derive(serde::Deserialize)]
#[allow(dead_code)]
pub struct UserEmail {
    name: String,
    email: String,
}
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(form: web::Form<UserEmail>, pool: web::Data<PgPool>) -> HttpResponse {
    let subscriber_name = SubscriberName::try_from(form.0.name.as_str());
    if subscriber_name.is_err() || form.0.email.is_empty() {
        return HttpResponse::BadRequest().finish();
    }
    let name = subscriber_name.unwrap();
    let new_subscriber = NewSubscriber {
        email: form.0.email,
        name,
    };

    match insert_subscriber(&new_subscriber, &pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, pool)
)]
async fn insert_subscriber(
    new_subscriber: &NewSubscriber,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        new_subscriber.email,
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Query Error: {:?}", e);
        e
    })?;
    Ok(())
}
