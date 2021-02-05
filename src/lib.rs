use actix_web;
use actix_web::dev;
use actix_web::web;
use serde;

#[derive(serde::Deserialize, serde::Serialize)]
struct FormData {
    email: String,
    name: String
}

// async fn greet(req: actix_web::HttpRequest) -> impl actix_web::Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}!", &name)
// }

async fn health_check() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().finish()
}

async fn subscribe(form: web::Form<FormData>) -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json2(&form.into_inner())
}

pub fn run(listener: std::net::TcpListener) -> Result<dev::Server, std::io::Error> {
    let server = actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
        .listen(listener)?
        .run();

    Ok(server)
}
