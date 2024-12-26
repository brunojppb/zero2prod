use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(form: web::Form<FormData>, pg_pool: web::Data<PgPool>) -> HttpResponse {
    let req_id = Uuid::new_v4();
    log::info!(
        "req_id={} Saving new subscriber email={} name={}",
        req_id,
        form.email,
        form.name
    );

    let result = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pg_pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            log::info!("req_id={} Subscriber saved", req_id);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!("req_id={} Failed to execute query: {:?}", req_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
