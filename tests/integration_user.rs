use reqwest::Client;
use serde_json::json;

#[actix_rt::test]
async fn test_create_and_delete_user() {
    let client = Client::new();

    // create a new user
    let create_res = client.post("http://localhost:8080/users")
        .json(&json!({ "name": "bob" }))
        .send()
        .await
        .unwrap();

    assert!(create_res.status().is_success());
    let user: serde_json::Value = create_res.json().await.unwrap();
    let user_id = user["id"].as_i64().unwrap();

    assert_eq!(user["name"], "bob");

    // cleanup: delete the created user
    let delete_res = client.delete(&format!("http://localhost:8080/users/{}", user_id))
        .send()
        .await
        .unwrap();

    assert!(delete_res.status().is_success());
}