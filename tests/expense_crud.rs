use reqwest::Client;
use serde_json::json;

#[actix_rt::test]
async fn expense_crud() {
  let client = Client::new();

  // create_expense
  let create_expense = client
    .post("http://localhost:8080/expenses")
    .json(&json!({
        "date": "2025-08-03T12:34:56",
        "amount": 123.45,
        "category": "food",
        "message": "lunch",
        "image_url": null,
        "longitude": 9.19,
        "latitude": 45.4642
    }))
    .send()
    .await
    .unwrap();
  assert!(create_expense.status().is_success());

  let expense: serde_json::Value = create_expense.json().await.unwrap();
  assert_eq!(expense["date"], "2025-08-03T12:34:56");
  assert!((expense["amount"].as_f64().unwrap() - 123.45).abs() < 1e-6);
  assert_eq!(expense["category"], "food");
  assert_eq!(expense["message"], "lunch");
  assert!(expense["image_url"].is_null());
  assert_eq!(expense["longitude"].as_f64().unwrap(), 9.19);
  assert_eq!(expense["latitude"].as_f64().unwrap(), 45.4642);

  let expense_id = expense["id"].as_i64().unwrap();

  // get_expenses
  let get_expenses = client.get("http://localhost:8080/expenses").send().await.unwrap();
  assert!(get_expenses.status().is_success());
  let expenses: Vec<serde_json::Value> = get_expenses.json().await.unwrap();
  assert!(expenses.iter().any(|e| e["id"].as_i64().unwrap() == expense_id));

  // get_expense
  let get_expense = client.get(&format!("http://localhost:8080/expenses/{}", expense_id)).send().await.unwrap();
  assert!(get_expense.status().is_success());

  let fetched_expense: serde_json::Value = get_expense.json().await.unwrap();
  assert_eq!(fetched_expense["id"].as_i64().unwrap(), expense_id);
  assert_eq!(fetched_expense["category"], "food");
  assert!((expense["amount"].as_f64().unwrap() - 123.45).abs() < 1e-6);
  assert_eq!(fetched_expense["date"].as_str().unwrap(), "2025-08-03T12:34:56");
  assert_eq!(fetched_expense["message"].as_str().unwrap(), "lunch");
  assert!(fetched_expense["image_url"].is_null());
  assert_eq!(fetched_expense["longitude"].as_f64().unwrap(), 9.1900); // example
  assert_eq!(fetched_expense["latitude"].as_f64().unwrap(), 45.4642); // example

  // update_expense
  let update_expense = client
    .put(&format!("http://localhost:8080/expenses/{}", expense_id))
    .json(&json!({
      "date": "2025-08-04T12:34:56",
      "amount": 150.00,
      "category": "dining",
      "message": "dinner",
      "image_url": null,
      "longitude": 9.1900,
      "latitude": 45.4642
    }))
    .send()
    .await
    .unwrap();

  assert!(update_expense.status().is_success());

  let updated_expense: serde_json::Value = update_expense.json().await.unwrap();
  assert_eq!(updated_expense["date"], "2025-08-04T12:34:56");
  assert!((updated_expense["amount"].as_f64().unwrap() - 150.00).abs() < 1e-6);
  assert_eq!(updated_expense["category"], "dining");
  assert_eq!(updated_expense["message"], "dinner");
  assert_eq!(updated_expense["longitude"], 9.1900);
  assert_eq!(updated_expense["latitude"], 45.4642);

  // delete_expense
  let delete_expense = client.delete(&format!("http://localhost:8080/expenses/{}", expense_id)).send().await.unwrap();
  assert!(delete_expense.status().is_success());

  let get_deleted_expense = client.get(&format!("http://localhost:8080/expenses/{}", expense_id)).send().await.unwrap();
  assert_eq!(get_deleted_expense.status(), reqwest::StatusCode::NOT_FOUND);

  // export_expenses_csv
  let exported_csv = client.get("http://localhost:8080/expenses/csv").send().await.unwrap();
  assert!(exported_csv.status().is_success());

  // export_expenses_md
  let exported_md = client.get("http://localhost:8080/expenses/md").send().await.unwrap();
  assert!(exported_md.status().is_success());
}
