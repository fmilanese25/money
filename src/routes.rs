use crate::handlers::expense;
use actix_web::web;

pub fn expense_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .route("/expenses", web::post().to(expense::create_expense))
    .route("/expenses", web::get().to(expense::get_expenses))
    .route("/expenses/{id}", web::get().to(expense::get_expense))
    .route("/expenses/{id}", web::put().to(expense::update_expense))
    .route("/expenses/{id}", web::delete().to(expense::delete_expense));
}

pub fn config(cfg: &mut web::ServiceConfig) {
  expense_routes(cfg);
}
