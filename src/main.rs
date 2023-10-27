use clap::Parser;
use rust_sql::{create_table, import_csv_to_sqlite, querydb};
use std::time::Instant; // Assuming your library is called "rust_sql"

#[derive(Parser, Debug)]
#[clap(name = "steam-cli", version = "0.1.0", author = "Keon Nartey")]
struct Arguments {
    #[clap(short, long)]
    query: String,
}

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
    let cl_input = Arguments::parse();

    let query = cl_input.query;

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
    querydb(&conn, &query).unwrap();
    
    profiler.stop("query db");
}
