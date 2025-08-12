use crate::models::{CreateUser, User};
use actix_web::{HttpResponse, Responder, web};
use sqlx::PgPool;

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

pub async fn get_users(pool: web::Data<PgPool>) -> impl Responder {
  let users = sqlx::query_as!(User, "select id, name from users")
    .fetch_all(pool.get_ref())
    .await
    .unwrap();

  HttpResponse::Ok().json(users)
}

pub async fn get_user(
  pool: web::Data<PgPool>,
  id_path: web::Path<i32>,
) -> impl Responder {
  let id = id_path.into_inner();

  let user =
    sqlx::query_as!(User, "select id, name from users where id = $1", id)
      .fetch_optional(pool.get_ref())
      .await;

  match user {
    Ok(Some(user)) => HttpResponse::Ok().json(user),
    Ok(None) => {
      HttpResponse::NotFound().body(format!("User with id {} not found", id))
    }
    Err(e) => {
      eprintln!("Failed to get user: {}", e);
      HttpResponse::InternalServerError().body("Failed to get user")
    }
  }
}

pub async fn update_user(
  pool: web::Data<PgPool>,
  id_path: web::Path<i32>,
  payload: web::Json<CreateUser>,
) -> impl Responder {
  let id = id_path.into_inner();

  let result = sqlx::query_as!(
    User,
    "update users set name = $1 where id = $2 returning id, name",
    payload.name,
    id
  )
  .fetch_optional(pool.get_ref())
  .await;

  match result {
    Ok(Some(user)) => HttpResponse::Ok().json(user),
    Ok(None) => {
      HttpResponse::NotFound().body(format!("User with id {} not found", id))
    }
    Err(e) => {
      eprintln!("Failed to update user: {}", e);
      HttpResponse::InternalServerError().body("Failed to update user")
    }
  }
}

pub async fn delete_user(
  pool: web::Data<PgPool>,
  id_path: web::Path<i32>,
) -> impl Responder {
  let id = id_path.into_inner();

  let result = sqlx::query!("delete from users where id = $1", id)
    .execute(pool.get_ref())
    .await;

  match result {
    Ok(res) if res.rows_affected() > 0 => {
      HttpResponse::Ok().body(format!("Deleted user with id {}", id))
    }
    Ok(_) => HttpResponse::NotFound().body("User not found"),
    Err(e) => {
      eprintln!("Failed to delete user: {}", e);
      HttpResponse::InternalServerError().body("Failed to delete user")
    }
  }
}
