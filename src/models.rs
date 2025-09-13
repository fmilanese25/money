use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, sqlx::FromRow)]
pub struct Expense {
  pub id: i32,
  pub date: NaiveDate,
  pub amount: f64,
  pub category: String,
  pub message: Option<String>,
  pub image_url: Option<String>,
  pub latitude: f64,
  pub longitude: f64,
}

#[derive(Deserialize)]
pub struct CreateExpense {
  pub date: NaiveDate,
  pub amount: f64,
  pub category: String,
  pub message: Option<String>,
  pub image_url: Option<String>,
  pub latitude: f64,
  pub longitude: f64,
}
