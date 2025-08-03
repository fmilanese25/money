use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::{User, CreateUser};

pub async fn get_users(pool: web::Data<PgPool>) -> impl Responder {
    let users = sqlx::query_as!(User, "select id, name from users")
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    HttpResponse::Ok().json(users)
}

pub async fn create_user(
    pool: web::Data<PgPool>,
    payload: web::Json<CreateUser>,
) -> impl Responder {
    let user = sqlx::query_as!(
        User,
        "insert into users (name) values ($1) returning id, name",
        payload.name
    )
    .fetch_one(pool.get_ref())
    .await
    .unwrap();

    HttpResponse::Ok().json(user)
}
