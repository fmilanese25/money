use actix_web::web;
use crate::handlers::{user, expense};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/users", web::get().to(user::get_users))
        .route("/users", web::post().to(user::create_user))
        .route("/expenses", web::get().to(expense::get_expenses))
        .route("/expenses", web::post().to(expense::create_expense))
        .route("/expenses/{id}", web::delete().to(expense::delete_expense));
}
