use rust_sql::{create_table, import_csv_to_sqlite, query_db};
use std::time::Instant; // Assuming your library is called "rust_sql"

struct Profiler {
    start_time: Instant,
}

impl Profiler {
    fn new() -> Profiler {
        Profiler {
            start_time: Instant::now(),
        }
    }

    fn start(&mut self) {
        self.start_time = Instant::now();
    }

    fn stop(&self, name: &str) {
        let elapsed = self.start_time.elapsed();
        println!("{} took {:?}", name, elapsed);
    }
}

fn main() {
    env_logger::init();
    let mut profiler = Profiler::new();
    profiler.start();

    let conn = rusqlite::Connection::open("Diabetes.db").unwrap();
    create_table(&conn).unwrap();

    profiler.stop("create table");
    profiler.start();

    import_csv_to_sqlite(&conn).unwrap();
    profiler.stop("import csv to sqlite");

    profiler.start();
    let q_result = query_db(&conn, "SELECT * FROM diabetes_data LIMIT 5").unwrap();
    println!("{:?}", q_result);
    profiler.stop("query db");
}
