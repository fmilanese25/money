mod db;
mod handlers;
mod models;
mod routes;
mod utils;

use actix_web::{App, HttpServer};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();
  let pool = db::init_db_pool().await;

  pprintln!("     Running at http://0.0.0.0:8080");

  HttpServer::new(move || App::new().app_data(actix_web::web::Data::new(pool.clone())).configure(routes::config))
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
