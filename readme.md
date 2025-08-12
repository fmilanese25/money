# money

backend for money app written in rust

## run
cargo run

## db
sudo service postgresql start

createdb -U francesco money
create database money
sqlx migrate run

psql -U francesco -d postgres

psql -U francesco -d money
pgcli -U francesco -d money

select * from users;
select * from expenses;

## curl tests

create a user  
```
curl -X POST http://localhost:8080/users \
  -H "Content-Type: application/json" \
  -d '{"name": "alice"}'
```
get all users  
```
curl http://localhost:8080/users
```

create an expense  
```
curl -X POST http://localhost:8080/expenses \
  -H "Content-Type: application/json" \
  -d '{"user_id":1, "date":"2025-08-03", "amount":12345, "category":"food", "message":"lunch", "image_url":null}'
```

get all expenses  
```
curl http://localhost:8080/expenses
```

## test
cargo test

## format
cargo fmt

## tech stack

- Rust
- PostgreSQL

