use crate::epprintln;
use crate::models::{CreateExpense, Expense};
use actix_web::{HttpResponse, Responder, web};
use sqlx::PgPool;

pub async fn create_expense(
  pool: web::Data<PgPool>,
  payload: web::Json<CreateExpense>,
) -> impl Responder {
  let amount_cents: i32 = (payload.amount * 100.0).round() as i32;

  let expense = sqlx::query_as!(
    Expense,
    r#"
    insert into expenses (date, amount, category, message, image_url, longitude, latitude)
    values ($1, $2, $3, $4, $5, $6, $7)
    returning id, date, amount, category, message, image_url, longitude, latitude
    "#,
    payload.date,
    amount_cents,
    payload.category,
    payload.message,
    payload.image_url,
    payload.longitude,
    payload.latitude,
  )
  .fetch_one(pool.get_ref())
  .await
  .unwrap();

  let response = Expense {
    amount: expense.amount as f64 / 100.0,
    ..expense
  };

  HttpResponse::Ok().json(response)
}

pub async fn get_expenses(pool: web::Data<PgPool>) -> impl Responder {
  let expenses = sqlx::query_as!(
    Expense,
    r#"
    select id, date, amount, category, message, image_url, longitude, latitude
    from expenses
    "#
  )
  .fetch_all(pool.get_ref())
  .await
  .unwrap();

  let response: Vec<_> = expenses
    .into_iter()
    .map(|e| Expense {
      amount: e.amount as f64 / 100.0,
      ..e
    })
    .collect();

  HttpResponse::Ok().json(response)
}

pub async fn get_expense(
  pool: web::Data<PgPool>,
  expense_id: web::Path<i32>,
) -> impl Responder {
  let expense_id = expense_id.into_inner();

  let expense = sqlx::query_as!(
    Expense,
    r#"
    select id, date, amount, category, message, image_url, longitude, latitude
    from expenses 
    where id = $1
    "#,
    expense_id
  )
  .fetch_optional(pool.get_ref())
  .await
  .unwrap();

  let response = expense.map(|e| Expense {
    amount: e.amount as f64 / 100.0,
    ..e
  });

  match response {
    Some(e) => HttpResponse::Ok().json(e),
    None => {
      epprintln!("error: get failed for expense with id {}, expense not found", expense_id);
      HttpResponse::NotFound().body(format!("error: get failed for expense with id {}, expense not found", expense_id))
    }
  }
}

pub async fn update_expense(
  pool: web::Data<PgPool>,
  id_path: web::Path<i32>,
  payload: web::Json<CreateExpense>,
) -> impl Responder {
  let expense_id = id_path.into_inner();
  let amount_cents: i32 = (payload.amount * 100.0).round() as i32;

  let result = sqlx::query_as!(
    Expense,
    r#"
    update expenses 
    set date = $2, amount = $3, category = $4, message = $5, image_url = $6, longitude = $7, latitude = $8
    where id = $1
    returning id, date, amount, category, message, image_url, longitude, latitude
    "#,
    expense_id,
    payload.date,
    amount_cents,
    payload.category,
    payload.message,
    payload.image_url,
    payload.longitude,
    payload.latitude,
  )
  .fetch_optional(pool.get_ref())
  .await;

  match result {
    Ok(Some(expense)) => {
      let response = Expense {
        amount: expense.amount as f64 / 100.0,
        ..expense
      };
      HttpResponse::Ok().json(response)
    }
    Ok(None) => {
      epprintln!("error: update failed for expense with id {}, expense not found", expense_id);
      HttpResponse::NotFound().body(format!(
        "error: update failed for expense with id {}, expense not found",
        expense_id
      ))
    }
    Err(e) => {
      epprintln!("error: update failed for expense with id {}: {}", expense_id, e);
      HttpResponse::InternalServerError().body(format!("error: update failed for expense with id {}: {}", expense_id, e))
    }
  }
}

pub async fn delete_expense(
  pool: web::Data<PgPool>,
  id_path: web::Path<i32>,
) -> impl Responder {
  let expense_id = id_path.into_inner();
  let result = sqlx::query!("delete from expenses where id = $1", expense_id).execute(pool.get_ref()).await;

  match result {
    Ok(r) if r.rows_affected() > 0 => HttpResponse::NoContent().finish(),
    Ok(_) => {
      epprintln!("error: delete failed for expense with id {}, expense not found", expense_id);
      HttpResponse::NotFound().body(format!(
        "error: delete failed for expense with id {}, expense not found",
        expense_id
      ))
    }
    Err(e) => {
      epprintln!("error: delete failed for expense with id {}: {}", expense_id, e);
      HttpResponse::InternalServerError().body(format!("error: update failed for expense with id {}: {}", expense_id, e))
    }
  }
}

pub async fn export_expenses_csv(pool: actix_web::web::Data<sqlx::PgPool>) -> impl Responder {
    let expenses = sqlx::query_as!(
        crate::models::Expense,
        r#"
        select id, date, amount, category, message, image_url, longitude, latitude from expenses
        "#
    )
    .fetch_all(pool.get_ref())
    .await
    .unwrap();

    let mut wtr = csv::Writer::from_writer(vec![]);
    for e in &expenses {
        wtr.serialize(e).unwrap();
    }
    let data = wtr.into_inner().unwrap();

    HttpResponse::Ok()
        .content_type("text/csv")
        .append_header(("Content-Disposition", "attachment; filename=expenses.csv"))
        .body(data)
}

pub async fn export_expenses_md(pool: actix_web::web::Data<sqlx::PgPool>) -> impl Responder {
    let expenses = sqlx::query_as!(
        crate::models::Expense,
        r#"
        select id, date, amount, category, message, image_url, longitude, latitude from expenses
        "#
    )
    .fetch_all(pool.get_ref())
    .await
    .unwrap();

    let mut lines = vec![];
    for e in expenses {
        let line = format!(
            "{}\t{}\t{:.2}\t{}\t{}\t{}\t{}",
            e.id,
            e.date,
            e.amount,
            e.category,
            e.message.unwrap_or_default(),
            e.latitude.map_or("".to_string(), |v| v.to_string()),
            e.longitude.map_or("".to_string(), |v| v.to_string()),
        );
        lines.push(line);
    }

    HttpResponse::Ok()
        .content_type("text/plain")
        .append_header(("Content-Disposition", "attachment; filename=expenses.md"))
        .body(lines.join("\n"))
}