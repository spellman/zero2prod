use actix_web;
use actix_web::web;

// async fn greet(req: actix_web::HttpRequest) -> impl actix_web::Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}!", &name)
// }

async fn health_check() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok()
}

pub async fn run() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .route("/health_check", web::get().to(health_check))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}