use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, sqlx::FromRow)]
pub struct User {
  pub id: i32,
  pub name: String,
}

#[derive(Deserialize)]
pub struct CreateUser {
  pub name: String,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct Expense {
  pub id: i32,
  pub user_id: i32,
  pub date: NaiveDate,
  pub amount: i32,
  pub category: String,
  pub message: Option<String>,
  pub image_url: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateExpense {
  pub user_id: i32,
  pub date: NaiveDate,
  pub amount: i32,
  pub category: String,
  pub message: Option<String>,
  pub image_url: Option<String>,
}
