use crate::routes;
use actix_web;
use actix_web::dev;
use actix_web::web;

pub fn run(listener: std::net::TcpListener) -> Result<dev::Server, std::io::Error> {
    let server = actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .route("/health_check", web::get().to(routes::health_check))
            .route("/subscriptions", web::post().to(routes::subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
