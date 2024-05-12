use rusqlite::Connection;

enum Task {
    Priority(u8),
    Description(String),
}

const DB_PATH: &str = "./todo_list.db3";

pub struct TodoList {
    db_conn: Connection,
}

impl TodoList {
    pub fn build() -> Result<TodoList, &'static str> {
        match Connection::open(DB_PATH) {
            Ok(db_conn) => Ok(TodoList { db_conn }),
            Err(_) => Err("Database connection failed"),
        }
    }

    pub fn add(&self) {}
    pub fn remove(&self) {}
    pub fn done(&self) {}
    pub fn list(&self) {}
}
