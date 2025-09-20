mod db;
mod handlers;
mod models;
mod routes;
mod utils;

use crate::utils::pretty::pretty;
use actix_web::{App, HttpServer};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();
  let pool = db::init_db_pool().await;

  println!("{}", pretty("     Running at http://0.0.0.0:8080"));

  // println!("{}", pretty("error: something went *wrong* here"));
  // println!("{}", pretty("info: bip boop"));
  // println!("{}", pretty("warning: check the *value*!"));
  // println!("{}", pretty("note: just a *tip* for you"));

  HttpServer::new(move || App::new().app_data(actix_web::web::Data::new(pool.clone())).configure(routes::config))
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
