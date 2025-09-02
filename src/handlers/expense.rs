use crate::models::{CreateExpense, Expense};
use actix_web::{HttpResponse, Responder, web};
use sqlx::PgPool;

pub async fn create_expense(
  pool: web::Data<PgPool>,
  payload: web::Json<CreateExpense>,
) -> impl Responder {
  let expense = sqlx::query_as!(
    Expense,
    "insert into expenses (date, amount, category, message, image_url)
         values ($1, $2, $3, $4, $5)
         returning id, date, amount, category, message, image_url",
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

pub async fn get_expenses(pool: web::Data<PgPool>) -> impl Responder {
  let expenses = sqlx::query_as!(
    Expense,
    "select id, date, amount, category, message, image_url from expenses"
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
        "select id, date, amount, category, message, image_url from expenses where id = $1",
        *expense_id
    )
    .fetch_optional(pool.get_ref())
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
        "update expenses set date = $1, amount = $2, category = $3, message = $4, image_url = $5
         where id = $6
         returning id, date, amount, category, message, image_url",
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
