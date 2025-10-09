# money
backend for money app written in rust

## run
```sh
cargo run
```

## db
```sh
# start_db
sudo service postgresql start

# reset_db
psql -U francesco -d postgres
drop database money;
create database money;
sqlx migrate run

# query_db
psql -U francesco -d money

select * from expenses;
truncate table expenses restart identity;
```

## curl -s tests
```sh
# create_expense  
curl -s -X POST http://localhost:8080/expenses \
  -H "Content-Type: application/json" \
  -d '{
    "date": "2025-08-03T12:34:56",
    "amount":123.45,
    "category":"food",
    "message":"lunch",
    "image_url":null,
    "latitude":41.9028,
    "longitude":12.4964
  }' | jq

# get_expenses  
curl -s http://localhost:8080/expenses | jq

# get_expense
curl -s http://localhost:8080/expenses/1 | jq

# update_expense
curl -s -X PUT http://localhost:8080/expenses/1 \
  -H "Content-Type: application/json" \
  -d '{
    "date": "2025-08-03T12:34:56",
    "amount":150.00,
    "category":"dining",
    "message":"dinner",
    "image_url":null,
    "latitude":41.9028,
    "longitude":12.4964
  }' | jq

# delete_expense
curl -s -X DELETE http://localhost:8080/expenses/1 | jq

# export_expenses_csv
curl -s http://localhost:8080/expenses/csv | cat
curl -s http://localhost:8080/expenses/csv -o expenses.csv

# export_expenses_md
curl -s http://localhost:8080/expenses/md | cat
curl -s http://localhost:8080/expenses/md -o expenses.md
```

## tests
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

