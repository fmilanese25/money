# money
backend for money app written in rust

## run
```sh
cargo run
```

## db
```sh
sudo service postgresql start

createdb -U francesco money
create database money
sqlx migrate run

psql -U francesco -d money
pgcli -U francesco -d money

select * from expenses;
truncate table expenses restart identity;



psql -U francesco -d postgres
drop database money;
create database money;
sqlx migrate run
```

## curl -s tests
```sh
# create expense  
curl -s -X POST http://localhost:8080/expenses \
  -H "Content-Type: application/json" \
  -d '{
    "date":"2025-08-03",
    "amount":123.45,
    "category":"food",
    "message":"lunch",
    "image_url":null,
    "latitude":41.9028,
    "longitude":12.4964
  }' | jq

# get all expenses  
curl -s http://localhost:8080/expenses | jq

# get expense
curl -s http://localhost:8080/expenses/1 | jq

# update expense
curl -s -X PUT http://localhost:8080/expenses/1 \
  -H "Content-Type: application/json" \
  -d '{
    "date":"2025-08-04",
    "amount":150.00,
    "category":"dining",
    "message":"dinner",
    "image_url":null,
    "latitude":41.9028,
    "longitude":12.4964
  }' | jq

# delete expense
curl -s -X DELETE http://localhost:8080/expenses/1 | jq
```

## test
```sh
cargo test
```

## format
```sh
cargo fmt
```

## tech stack
- Rust
- PostgreSQL

