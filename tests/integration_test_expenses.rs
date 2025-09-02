use reqwest::Client;
use serde_json::json;

#[actix_rt::test]
async fn integration_test_expenses() {
  let client = Client::new();

  // create expense
  let create_expense = client
    .post("http://localhost:8080/expenses")
    .json(&json!({
        "date": "2025-08-03",
        "amount": 12345,
        "category": "food",
        "message": "lunch",
        "image_url": null
    }))
    .send()
    .await
    .unwrap();
  assert!(create_expense.status().is_success());

  let expense: serde_json::Value = create_expense.json().await.unwrap();
  assert_eq!(expense["category"], "food");
  assert_eq!(expense["amount"], 12345);

  let expense_id = expense["id"].as_i64().unwrap();

  // get the expense
  let get_expense = client
    .get(&format!("http://localhost:8080/expenses/{}", expense_id))
    .send()
    .await
    .unwrap();
  assert!(get_expense.status().is_success());
  let fetched_expense: serde_json::Value = get_expense.json().await.unwrap();
  assert_eq!(fetched_expense["id"].as_i64().unwrap(), expense_id);
  assert_eq!(fetched_expense["category"], "food");

  // update expense
  let update_expense = client
    .put(&format!("http://localhost:8080/expenses/{}", expense_id))
    .json(&json!({
        "date": "2025-08-04",
        "amount": 15000,
        "category": "dining",
        "message": "dinner",
        "image_url": null
    }))
    .send()
    .await
    .unwrap();
  assert!(update_expense.status().is_success());
  let updated_expense: serde_json::Value = update_expense.json().await.unwrap();
  assert_eq!(updated_expense["amount"], 15000);
  assert_eq!(updated_expense["category"], "dining");
  assert_eq!(updated_expense["message"], "dinner");

  // delete expense
  let delete_expense = client
    .delete(&format!("http://localhost:8080/expenses/{}", expense_id))
    .send()
    .await
    .unwrap();
  assert!(delete_expense.status().is_success());
}
