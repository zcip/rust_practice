use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use env_logger;

#[get("/asyncio")]
async fn asyncio_weather() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env_logger::init_from_env()

    HttpServer::new(move || App::new().service(asyncio_weather))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
