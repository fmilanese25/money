use crate::models::{CreateExpense, Expense};
use actix_web::{HttpResponse, Responder, web};
use sqlx::PgPool;

pub async fn create_expense(
  pool: web::Data<PgPool>,
  payload: web::Json<CreateExpense>,
) -> impl Responder {
  let expense = sqlx::query_as!(
    Expense,
    "insert into expenses (user_id, date, amount, category, message, image_url)
         values ($1, $2, $3, $4, $5, $6)
         returning id, user_id, date, amount, category, message, image_url",
    payload.user_id,
    payload.date,
    payload.amount,
    payload.category,
    payload.message,
    payload.image_url
  )
  .fetch_one(pool.get_ref())
  .await
  .unwrap();

  HttpResponse::Ok().json(expense)
}

pub async fn get_expenses(
  pool: web::Data<PgPool>,
  user_id: web::Path<i32>,
) -> impl Responder {
  let expenses = sqlx::query_as!(
    Expense,
    "select id, user_id, date, amount, category, message, image_url 
       from expenses 
       where user_id = $1",
    *user_id
  )
  .fetch_all(pool.get_ref())
  .await
  .unwrap();

  HttpResponse::Ok().json(expenses)
}

pub async fn get_expense(
  pool: web::Data<PgPool>,
  expense_id: web::Path<i32>,
) -> impl Responder {
  let expense = sqlx::query_as!(
    Expense,
    "select id, user_id, date, amount, category, message, image_url 
       from expenses 
       where id = $1",
    *expense_id
  )
  .fetch_optional(pool.get_ref()) // optional so it won't panic if not found
  .await
  .unwrap();

  match expense {
    Some(e) => HttpResponse::Ok().json(e),
    None => HttpResponse::NotFound().finish(),
  }
}
pub async fn update_expense(
  pool: web::Data<PgPool>,
  id_path: web::Path<i32>,
  payload: web::Json<CreateExpense>,
) -> impl Responder {
  let id = id_path.into_inner();

  let result = sqlx::query_as!(
      Expense,
      "update expenses set user_id = $1, date = $2, amount = $3, category = $4, message = $5, image_url = $6 where id = $7 returning id, user_id, date, amount, category, message, image_url",
      payload.user_id,
      payload.date,
      payload.amount,
      payload.category,
      payload.message,
      payload.image_url,
      id
  )
  .fetch_optional(pool.get_ref())
  .await;

  match result {
    Ok(Some(expense)) => HttpResponse::Ok().json(expense),
    Ok(None) => {
      HttpResponse::NotFound().body(format!("expense with id {} not found", id))
    }
    Err(e) => {
      eprintln!("failed to update expense: {}", e);
      HttpResponse::InternalServerError().body("failed to update expense")
    }
  }
}

pub async fn delete_expense(
  pool: web::Data<PgPool>,
  id_path: web::Path<i32>,
) -> impl Responder {
  let expense_id = id_path.into_inner();
  let result = sqlx::query!("delete from expenses where id = $1", expense_id)
    .execute(pool.get_ref())
    .await;

  match result {
    Ok(r) if r.rows_affected() > 0 => HttpResponse::NoContent().finish(),
    Ok(_) => HttpResponse::NotFound().body("expense not found"),
    Err(e) => {
      eprintln!("failed to delete expense: {:?}", e);
      HttpResponse::InternalServerError().finish()
    }
  }
}
