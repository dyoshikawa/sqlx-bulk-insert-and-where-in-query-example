# Setup and Run

1. `docker-compose up -d`
2. `docker-compose exec mysql mysql -uroot -psecret dev` -> `mysql> source /create_people_table.sql;`
3. `cargo run`

# Expected output

```
result: 3
[Person { id: 1, name: "foo" }, Person { id: 2, name: "bar" }, Person { id: 3, name: "baz" }]
```
