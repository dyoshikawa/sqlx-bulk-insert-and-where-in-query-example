# Setup and Run

1. `docker-compose up -d`
2. `docker-compose exec mysql mysql -uroot -psecret dev` -> `mysql> source /create_people_table.sql;`
3. `cargo run`
