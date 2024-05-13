use rusqlite::Connection;

const DB_PATH: &str = "./todo_list.db3";

pub struct TodoList {
    db_conn: Connection,
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

    pub fn add(&self) -> Result<(), &'static str> {
        Ok(())
    }
    
    pub fn remove(&self) -> Result<(), &'static str> {
        Ok(())
    }
    
    pub fn done(&self) -> Result<(), &'static str> {
        Ok(())
    }
    
    pub fn list(&self) -> Result<(), &'static str> {
        Ok(())
    }
    
}
