use actix_web::{get, HttpResponse};

#[get("/healthcheck")]
pub async fn healthcheck() -> HttpResponse {
    HttpResponse::Ok().finish()
}
