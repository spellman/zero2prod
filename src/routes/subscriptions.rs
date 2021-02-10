use actix_web;
use actix_web::web;
use serde;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>) -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json2(&form.into_inner())
}
