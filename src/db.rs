use sqlx::postgres::PgPoolOptions;
use std::env;

pub async fn init_db_pool() -> sqlx::PgPool {
  let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  PgPoolOptions::new().max_connections(5).connect(&db_url).await.expect("failed to connect to db")
}
