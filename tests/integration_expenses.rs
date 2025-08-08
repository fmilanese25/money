// integration test for expense endpoints

use reqwest::Client;
use serde_json::json;

#[actix_rt::test]
async fn test_create_and_delete_expense() {
    let client = Client::new();

    // first create user to get user_id
    let user_res = client.post("http://localhost:8080/users")
        .json(&json!({ "name": "alice" }))
        .send()
        .await
        .unwrap();
    let user: serde_json::Value = user_res.json().await.unwrap();
    let user_id = user["id"].as_i64().unwrap();

    // create expense linked to user_id
    let expense_res = client.post("http://localhost:8080/expenses")
        .json(&json!({
            "user_id": user_id,
            "date": "2025-08-03",
            "amount": 12345,
            "category": "food",
            "message": "lunch",
            "image_url": null
        }))
        .send()
        .await
        .unwrap();

    assert!(expense_res.status().is_success());
    let expense: serde_json::Value = expense_res.json().await.unwrap();
    assert_eq!(expense["category"], "food");
    assert_eq!(expense["amount"], 12345);

    let expense_id = expense["id"].as_i64().unwrap();

    // cleanup: delete the created expense
    let delete_expense = client.delete(&format!("http://localhost:8080/expenses/{}", expense_id))
        .send()
        .await
        .unwrap();
    assert!(delete_expense.status().is_success());

    // cleanup: delete the created user
    let delete_user = client.delete(&format!("http://localhost:8080/users/{}", user_id))
        .send()
        .await
        .unwrap();
    assert!(delete_user.status().is_success());
}
