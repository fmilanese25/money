use reqwest::Client;
use serde_json::json;

#[actix_rt::test]
async fn integration_test_user() {
  let client = Client::new();

  // create a new user
  let create_user = client
    .post("http://localhost:8080/users")
    .json(&json!({ "name": "bob" }))
    .send()
    .await
    .unwrap();
  assert!(create_user.status().is_success());
  let user: serde_json::Value = create_user.json().await.unwrap();
  let user_id = user["id"].as_i64().unwrap();
  assert_eq!(user["name"], "bob");

  // get the user by user_id
  let get_user = client
    .get(&format!("http://localhost:8080/users/{}", user_id))
    .send()
    .await
    .unwrap();
  assert!(get_user.status().is_success());
  let user_get: serde_json::Value = get_user.json().await.unwrap();
  assert_eq!(user_get["id"], user_id);
  assert_eq!(user_get["name"], "bob");

  // get all the users
  let get_all = client.get("http://localhost:8080/users").send().await.unwrap();
  assert!(get_all.status().is_success());
  let users: Vec<serde_json::Value> = get_all.json().await.unwrap();
  assert!(users.iter().any(|user| user["id"].as_i64() == Some(user_id as i64)));

  // update the user by user_id
  let update_user = client
    .put(&format!("http://localhost:8080/users/{}", user_id))
    .json(&json!({ "name": "robert" }))
    .send()
    .await
    .unwrap();
  assert!(update_user.status().is_success());
  let updated_user: serde_json::Value = update_user.json().await.unwrap();
  assert_eq!(updated_user["name"], "robert");

  // delete the user by user_id
  let delete_user = client
    .delete(&format!("http://localhost:8080/users/{}", user_id))
    .send()
    .await
    .unwrap();
  assert!(delete_user.status().is_success());

  // confirm user no longer exists
  let get_deleted = client
    .get(&format!("http://localhost:8080/users/{}", user_id))
    .send()
    .await
    .unwrap();
  assert_eq!(get_deleted.status(), 404);
}
