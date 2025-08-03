use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::{Expense, CreateExpense};

pub async fn get_expenses(pool: web::Data<PgPool>) -> impl Responder {
    let expenses = sqlx::query_as!(
        Expense,
        "select id, user_id, date, amount, category, message, image_url from expenses"
    )
    .fetch_all(pool.get_ref())
    .await
    .unwrap();

    HttpResponse::Ok().json(expenses)
}

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
