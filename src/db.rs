use rusqlite::{Connection, Result, params};
use sha2::{Sha256, Digest};

pub fn init() -> Result<()> {
    let conn = Connection::open("./test.db")?;

    match conn.execute(
        "CREATE TABLE IF NOT EXISTS urls (
            id      VARCHAR(8) NOT NULL PRIMARY KEY,
            url     TEXT NOT NULL
            );",
        [],
        ) {
        Ok(updated) => println!("new table created: {}", updated),
        Err(err) => println!("error creating table: {}", err),
    }
    Ok(())
}

pub fn put(url: String) -> Result<String> {
    let id: String = format!("{:x}", Sha256::new()
        .chain_update(url.clone())
        .finalize())
        .chars()
        .take(4)
        .collect();
    let conn = Connection::open("./test.db")?;
    match conn.execute(
        "INSERT INTO urls (id, url) VALUES (?1, ?2);",
        params![id, url]) {
        Ok(updated) => println!("Added {} <-> {}, {} rows updated", url, id, updated),
        Err(err) => println!("error: {}", err),
    }
    Ok(id)
}

pub fn get(id: String) -> Result<String> {
    let conn = Connection::open("./test.db")?;
    conn.query_row(
        "SELECT url FROM urls WHERE id = ?1;",
        params![id],
        |row| row.get(0)
    )
}
