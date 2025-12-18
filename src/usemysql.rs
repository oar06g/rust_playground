use mysql::*;
use mysql::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct User {
    id: i32,
    name: String,
    age: i32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "mysql://root:secret@localhost:3306/rust_db";

    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;
    
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS users (
            id INT AUTO_INCREMENT PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            age INT NOT NULL
        )"
    )?;

    let users = vec![
        User { id: 0, name: "Alice".to_string(), age: 30 },
        User { id: 0, name: "Bob".to_string(), age: 25 },
        User { id: 0, name: "Charlie".to_string(), age: 35 },
    ];

    conn.exec_batch(
        r"INSERT INTO users (name, age) VALUES (:name, :age)",
        users.iter().map(|user| params! {
            "name" => &user.name,
            "age" => user.age,
        }),
    )?;
    let selected_users: Vec<User> = conn.query_map(
        "SELECT id, name, age FROM users",
        |(id, name, age)| User { id, name, age }
    )?;

    for user in selected_users {
        println!("User: {} - {} - {}", user.id, user.name, user.age);
    }

    Ok(())

}