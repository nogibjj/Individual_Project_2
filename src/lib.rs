use rusqlite::{Connection, Result};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn connect_db(db_name: &str) -> Result<Connection> {
    let conn = Connection::open(db_name)?;
    Ok(conn)
}

pub fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS DiabetesDB (Column1 TEXT, Column2 TEXT, Column3 TEXT, Column4 TEXT, Column5 TEXT, Column6 TEXT, Column7 TEXT, Column8 TEXT, Column9 TEXT)",
        [],
    )?;
    Ok(())
}

pub fn load_data(conn: &Connection, dataset_path: &str) -> Result<(), rusqlite::Error> {
    let file = File::open(dataset_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let record = line?;
        let record: Vec<&dyn rusqlite::ToSql> = record.split(',').map(|s| s.trim()).collect();

        conn.execute(
            "INSERT INTO DiabetesDB (Column1, Column2, Column3, Column4, Column5, Column6, Column7, Column8, Column9) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            record.as_slice(),
        )?;
    }

    Ok(())
}
