use actix_web::{get, middleware, web, App, HttpResponse, HttpServer};
use env_logger;
use futures_util::future::join_all;
use log;

mod db;
use db::{Pool, Queries};
use r2d2_sqlite::SqliteConnectionManager;

#[get("/asyncio")]
async fn asyncio_weather(db: web::Data<Pool>) -> Result<HttpResponse, actix_web::Error> {
    let result = vec![
        db::execute(&db, Queries::GetTopTenHottestYears).await?,
        db::execute(&db, Queries::GetTopTenColdestYears).await?,
        db::execute(&db, Queries::GetTopTenHottestMonth).await?,
        db::execute(&db, Queries::GetTopTenColdestMonth).await?,
    ];
    Ok(HttpResponse::Ok().json(result))
}

#[get("/parallel")]
async fn parallel_weather(db: web::Data<Pool>) -> Result<HttpResponse, actix_web::Error> {
    let future_result = vec![
        db::execute(&db, Queries::GetTopTenHottestYears),
        db::execute(&db, Queries::GetTopTenColdestYears),
        db::execute(&db, Queries::GetTopTenHottestMonth),
        db::execute(&db, Queries::GetTopTenColdestMonth),
    ];

    let result: Result<Vec<_>, _> = join_all(future_result).await.into_iter().collect();
    Ok(HttpResponse::Ok().json(result.map_err(actix_web::Error::from)?))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let manager = SqliteConnectionManager::file("weather.db");
    let pool = Pool::new(manager).unwrap();

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(asyncio_weather)
            .service(parallel_weather)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
