use rusqlite::Connection;

const DB_PATH: &str = "./todo_list.db3";

pub struct TodoList {
    db_conn: Connection,
}

#[derive(Debug)]
struct Task {
    id: u8,
    description: String,
}

impl TodoList {
    pub fn build() -> Result<TodoList, &'static str> {
        match Connection::open(DB_PATH) {
            Ok(db_conn) => {
                let _ = db_conn.execute(
                    "create table if not exists tasks (
                        id integer primary key,
                        description text not null unique
                    )",
                    (),
                );
                Ok(TodoList { db_conn })
            }
            Err(_) => Err("Database connection failed"),
        }
    }

    pub fn add(&self, descriptions: &Vec<String>) -> Result<(), &'static str> {
        let mut count = 0;
        for desc in descriptions {
            match self
                .db_conn
                .execute("INSERT INTO tasks (description) values (?1)", [desc])
            {
                Ok(_) => count += 1,
                Err(_) => return Err("Insertion failed"),
            }
        }

        println!("{} rows were inserted", count);
        Ok(())
    }

    pub fn remove(&self, ids: &Vec<String>) -> Result<(), &'static str> {
        let mut count = 0;
        for id in ids {
            match self
                .db_conn
                .execute("DELETE FROM tasks WHERE id = (?1)", [id])
            {
                Ok(_) => count += 1,
                Err(_) => return Err("Delete failed"),
            }
        }

        println!("{} rows were removed", count);
        Ok(())
    }

    pub fn list(&self) -> Result<(), &'static str> {
        let mut stmt = match self.db_conn.prepare("SELECT * FROM tasks") {
            Ok(stmt) => stmt,
            Err(_) => return Err("Recovering data failed"),
        };

        let tasks = stmt
            .query_map((), |row| {
                Ok(Task {
                    id: row.get(0)?,
                    description: row.get(1)?,
                })
            })
            .unwrap();

        for task in tasks {
            match task {
                Ok(t) => println!("{} {}", t.id, t.description),
                Err(_) => return Err("Error reading data"),
            }
        }

        Ok(())
    }
}
