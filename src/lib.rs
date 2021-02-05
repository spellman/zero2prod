use actix_web;
use actix_web::dev;
use actix_web::web;

// async fn greet(req: actix_web::HttpRequest) -> impl actix_web::Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}!", &name)
// }

async fn health_check() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().finish()
}

pub fn run(listener: std::net::TcpListener) -> Result<dev::Server, std::io::Error> {
    let server = actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .route("/health_check", web::get().to(health_check))
    })
        .listen(listener)?
        .run();

    Ok(server)
}
