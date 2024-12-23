use actix_web::{dev::Server, web, App, HttpServer};

use crate::routes::{health_check::health_check, subscriptions::subscribe};

pub fn run(listener: std::net::TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
