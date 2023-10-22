use rust_sql::{connect_db, create_table, load_data};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_name = "src/Diabetes.db"; // Adjust this path if needed
    let dataset_path = "src/Diabetes.csv"; // Adjust this path if needed

    let conn = connect_db(db_name)?;
    create_table(&conn)?;
    load_data(&conn, dataset_path)?;

    Ok(())
}
