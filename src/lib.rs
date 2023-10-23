use csv::Reader;
use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct DiabetesData {
    pub pregnancies: i32,
    pub glucose: i32,
    pub blood_pressure: i32,
    pub skin_thickness: i32,
    pub insulin: i32,
    pub bmi: f64,
    pub diabetes_pedigree_function: f64,
    pub age: i32,
    pub outcome: i32,
}

pub fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS diabetes_data (
             Pregnancies INTEGER,
             Glucose INTEGER,
             BloodPressure INTEGER,
             SkinThickness INTEGER,
             Insulin INTEGER,
             BMI REAL,
             DiabetesPedigreeFunction REAL,
             Age INTEGER,
             Outcome INTEGER
        )",
        [],
    )?;
    Ok(())
}

pub fn import_csv_to_sqlite(conn: &Connection) -> Result<()> {
    let mut reader = match Reader::from_path("/workspaces/individual_project_2/src/Diabetes.csv") {
        Ok(reader) => reader,
        Err(err) => {
            println!("Error reading file: {:?}", err);
            return Ok(());
        }
    };

    let mut batch_data = String::new();

    for result in reader.records() {
        let record = match result {
            Ok(record) => record,
            Err(err) => {
                println!("Error reading record: {:?}", err);
                continue;
            }
        };

        let pregnancies = record[0].parse::<i32>().unwrap();
        let glucose = record[1].parse::<i32>().unwrap();
        let blood_pressure = record[2].parse::<i32>().unwrap();
        let skin_thickness = record[3].parse::<i32>().unwrap();
        let insulin = record[4].parse::<i32>().unwrap();
        let bmi = record[5].parse::<f64>().unwrap();
        let diabetes_pedigree_function = record[6].parse::<f64>().unwrap();
        let age = record[7].parse::<i32>().unwrap();
        let outcome = record[8].parse::<i32>().unwrap();

        let curr_query = format!(
            "INSERT INTO diabetes_data (Pregnancies, Glucose, BloodPressure, SkinThickness, Insulin, BMI, DiabetesPedigreeFunction, Age, Outcome) VALUES ({}, {}, {}, {}, {}, {}, {}, {}, {});",
            pregnancies, glucose, blood_pressure, skin_thickness, insulin, bmi, diabetes_pedigree_function, age, outcome
        );
        batch_data.push_str(&curr_query);

        const BATCH_SIZE: usize = 9999999999999999999;
        if batch_data.len() >= BATCH_SIZE {
            conn.execute_batch(&batch_data)?;
            batch_data.clear();
        }
    }

    if !batch_data.is_empty() {
        conn.execute_batch(&batch_data)?;
    }

    Ok(())
}

pub fn query_db(conn: &Connection, query: &str) -> Result<Vec<DiabetesData>> {
    let mut stmt = conn.prepare(query)?;

    let rows = stmt.query_map([], |row| {
        Ok(DiabetesData {
            pregnancies: row.get("Pregnancies")?,
            glucose: row.get("Glucose")?,
            blood_pressure: row.get("BloodPressure")?,
            skin_thickness: row.get("SkinThickness")?,
            insulin: row.get("Insulin")?,
            bmi: row.get("BMI")?,
            diabetes_pedigree_function: row.get("DiabetesPedigreeFunction")?,
            age: row.get("Age")?,
            outcome: row.get("Outcome")?,
        })
    })?;

    let diabetes_data: Result<Vec<DiabetesData>> = rows.collect();
    diabetes_data
}
