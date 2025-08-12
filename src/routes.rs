use crate::handlers::{expense, user};
use actix_web::web;

pub fn user_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .route("/users", web::post().to(user::create_user))
    .route("/users", web::get().to(user::get_users))
    .route("/users/{id}", web::get().to(user::get_user))
    .route("/users/{id}", web::put().to(user::update_user))
    .route("/users/{id}", web::delete().to(user::delete_user));
}

pub fn expense_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .route("/expenses", web::post().to(expense::create_expense))
    .route("/expenses", web::get().to(expense::get_expenses))
    .route("/expenses/{id}", web::get().to(expense::get_expense))
    .route("/expenses/{id}", web::put().to(expense::update_expense))
    .route("/expenses/{id}", web::delete().to(expense::delete_expense));
}

pub fn config(cfg: &mut web::ServiceConfig) {
  user_routes(cfg);
  expense_routes(cfg);
}
