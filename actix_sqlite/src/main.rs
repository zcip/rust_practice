use actix_web::{get, middleware, App, HttpResponse, HttpServer, Responder};
use env_logger;
use log;

#[get("/asyncio")]
async fn asyncio_weather() -> impl Responder {
    let result = vec!["a", "b", "c"];
    HttpResponse::Ok().json(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(asyncio_weather)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
