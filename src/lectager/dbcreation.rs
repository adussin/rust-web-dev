use sqlite;

use crate::lectager::constants::*;
pub fn create_db(){
    let conn = sqlite::open(DBNAME).unwrap();
    let mut query = "CREATE TABLE IF NOT EXISTS Student (id TEXT PRIMARY KEY, name TEXT, surname TEXT);";
    conn.execute(query).unwrap();
    query = "CREATE TABLE IF NOT EXISTS Lecture (
                when_date DATE NOT NULL, start TIME NOT NULL, end TIME NOT NULL,
                student_id TEXT NOT NULL,
                is_done BOOL NOT NULL,
                hourly_rate REAL NOT NULL,
                is_paid BOOL NOT NULL,
                UNIQUE(when_date, start, end),
                FOREIGN KEY(student_id) REFERENCES Student(id)
            );";
    conn.execute(query).unwrap();
}

pub fn populate(){
    let names = &["Alice", "Beatrix", "Charlotte", "Diana", "Emily"];
    let surnames = &["One", "Two", "Three", "Four", "Five"];
    let conn = sqlite::open(DBNAME).unwrap();
    for pair in names.iter().zip(surnames.iter()){
        let query = format!("INSERT OR IGNORE INTO Student (id, name, surname) VALUES ('{}{}', '{}', '{}')", pair.0, pair.1, pair.0, pair.1);
        conn.execute(query).unwrap();
    }
}