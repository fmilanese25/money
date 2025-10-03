use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, sqlx::FromRow)]
pub struct Expense {
  pub id: i32,
  pub date: NaiveDateTime,
  pub amount: f64,
  pub category: String,
  pub message: Option<String>,
  pub image_url: Option<String>,
  pub latitude: Option<f64>,
  pub longitude: Option<f64>,
}

#[derive(Deserialize)]
pub struct CreateExpense {
  pub date: NaiveDateTime,
  pub amount: f64,
  pub category: String,
  pub message: Option<String>,
  pub image_url: Option<String>,
  pub latitude: Option<f64>,
  pub longitude: Option<f64>,
}
