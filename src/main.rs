use sqlx::{query::query, Cursor, MySqlPool, Row};

#[derive(Debug)]
struct Person {
    id: u64,
    name: String,
}

#[tokio::main]
async fn main() {
    let pool = MySqlPool::builder()
        .max_size(5)
        .build("mysql://root:secret@localhost:3306/dev")
        .await
        .unwrap();

    let names = vec!["foo".to_string(), "bar".to_string(), "baz".to_string()];

    // example: BULK INSERT
    let mut sql = "
INSERT INTO people(name) VALUES
"
    .to_string();
    for (i, _) in names.iter().enumerate() {
        sql.push_str("(?)");
        if i < names.len() - 1 {
            sql.push_str(", ");
        }
    }
    let mut q = query(sql.as_str());
    for n in names.iter() {
        q = q.bind(n);
    }
    let res = q.execute(&pool).await.unwrap();
    println!("result: {}", res);

    // example: WHERE IN
    let mut sql = "SELECT * FROM people WHERE name IN (".to_string();
    for (i, _) in names.iter().enumerate() {
        sql.push_str("?");
        if i < names.len() - 1 {
            sql.push_str(", ");
        } else {
            sql.push_str(")")
        }
    }
    let mut people = vec![];
    let mut q = query(sql.as_str());
    for n in names.iter() {
        q = q.bind(n);
    }
    let mut cursor = q.fetch(&pool);
    while let Some(row) = cursor.next().await.unwrap() {
        people.push(Person {
            id: row.get("id"),
            name: row.get("name"),
        });
    }
    println!("{:?}", people)
}
