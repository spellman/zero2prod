use actix_web::web;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>) -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json2(&form.into_inner())
}
