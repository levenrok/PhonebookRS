use rusqlite::{Connection, Result};

struct Contact {
    name: String,
    number: u32,
    email: Option<String>,
    address: Option<String>,
}

fn main() -> Result<()> {
    let conn = Connection::new("db/phonebook.db")?;

    Ok(())
}
