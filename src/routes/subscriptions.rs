use actix_web::{web, Responder, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
  HttpResponse::Ok().finish()
}