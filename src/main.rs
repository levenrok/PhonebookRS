use rusqlite::{Connection, Result};

struct Contact {
    name: String,
    number: u32,
    email: Option<String>,
    address: Option<String>,
}

fn main() -> Result<()> {
    let conn = Connection::open("src/db/phonebook.db")?;

    create_table(&conn)?;

    Ok(())
}

fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS phonebook (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            number UNSIGNED INT,
            email TEXT,
            address TEXT
        )",
        [],
    )?;

    Ok(())
}