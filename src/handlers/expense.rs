use crate::epprintln;
use crate::models::{CreateExpense, Expense};
use actix_multipart::Multipart;
use actix_web::{HttpResponse, Responder, web};
use csv::ReaderBuilder;
use futures_util::TryStreamExt;
use sqlx::PgPool;
use std::io::Cursor;

pub async fn create_expense(
  pool: web::Data<PgPool>,
  payload: web::Json<CreateExpense>,
) -> impl Responder {
  let amount_cents: i32 = (payload.amount * 100.0).round() as i32;

  let expense = sqlx::query_as!(
    Expense,
    r#"
    insert into expenses (date, amount, category, image_url, longitude, latitude, message)
    values ($1, $2, $3, $4, $5, $6, $7)
    returning id, date, amount, category, image_url, longitude, latitude, message
    "#,
    payload.date,
    amount_cents,
    payload.category,
    payload.image_url,
    payload.longitude,
    payload.latitude,
    payload.message,
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
    select id, date, amount, category, image_url, longitude, latitude, message
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
    select id, date, amount, category, image_url, longitude, latitude, message
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
    set date = $2, amount = $3, category = $4, image_url = $5, longitude = $6, latitude = $7, message = $8
    where id = $1
    returning id, date, amount, category, image_url, longitude, latitude, message
    "#,
    expense_id,
    payload.date,
    amount_cents,
    payload.category,
    payload.image_url,
    payload.longitude,
    payload.latitude,
    payload.message,
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
      HttpResponse::InternalServerError().body(format!("error: delete failed for expense with id {}: {}", expense_id, e))
    }
  }
}

pub async fn export_expenses_csv(pool: actix_web::web::Data<sqlx::PgPool>) -> impl Responder {
  let expenses = sqlx::query_as!(
    Expense,
    r#"
    select id, date, amount, category, image_url, longitude, latitude, message
    from expenses
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

  HttpResponse::Ok().content_type("text/csv").append_header(("Content-Disposition", "attachment; filename=expenses.csv")).body(data)
}
pub async fn import_expenses_csv(
  pool: web::Data<sqlx::PgPool>,
  mut payload: Multipart,
) -> impl Responder {
  let mut imported = 0u32;

  while let Ok(Some(mut field)) = payload.try_next().await {
    let mut buffer = Vec::new();

    while let Ok(Some(chunk)) = field.try_next().await {
      buffer.extend_from_slice(&chunk);
    }

    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(Cursor::new(buffer));

    for result in reader.deserialize::<Expense>() {
      if let Ok(expense) = result {
        let amount_cents: i32 = (expense.amount * 100.0).round() as i32;
        let res = sqlx::query!(
          r#"
          insert into expenses (id, date, amount, category, image_url, longitude, latitude, message)
          values ($1, $2, $3, $4, $5, $6, $7, $8)
          on conflict (id) do nothing
          "#,
          expense.id,
          expense.date,
          amount_cents,
          expense.category,
          expense.image_url,
          expense.longitude,
          expense.latitude,
          expense.message
        )
        .execute(pool.get_ref())
        .await;

        match res {
          Ok(_) => {
            println!(
              "note: create expense success  id {}   date {:?}   amount {:?}   category={:?}",
              expense.id, expense.date, expense.amount, expense.category
            );
            imported += 1;
          }
          Err(err) => eprintln!("error: create expense failed {}", err),
        }
      }
    }
  }

  HttpResponse::Ok().body(format!("note: created {} expenses", imported))
}

pub async fn export_expenses_md(pool: actix_web::web::Data<sqlx::PgPool>) -> impl Responder {
  let expenses = sqlx::query_as!(
    Expense,
    r#"
    select id, date, amount, category, image_url, longitude, latitude, message
    from expenses
    "#
  )
  .fetch_all(pool.get_ref())
  .await
  .unwrap();

  let mut lines = vec![];

  let header = format!(
    "{:>5}   {:>19}   {:>8}   {:>10}   {:>23}   {:>20}",
    "id", "date     time", "amount", "category", "latitude    longitude", "message"
  );
  lines.push(header);

  for e in expenses {
    let lat_lon = match (e.latitude, e.longitude) {
      (Some(lat), Some(lon)) => format!("{:9.6}  {:9.6}", lat, lon),
      _ => "".to_string(),
    };

    let line = format!(
      "{:>5}   {:>19}   {:>8.2}   {:>10}   {:>23}   {}",
      e.id,
      e.date,
      e.amount,
      e.category,
      lat_lon,
      e.message.unwrap_or_default()
    );
    lines.push(line);
  }

  HttpResponse::Ok()
    .content_type("text/plain")
    .append_header(("Content-Disposition", "attachment; filename=expenses.md"))
    .body(lines.join("\n"))
}
