use sqlite;

pub fn create_db(){
    let conn = sqlite::open(":lectager.db:").unwrap();
    let query = "CREATE TABLE IF NOT EXISTS Student (id TEXT PRIMARY KEY, name TEXT, surname TEXT);";
    conn.execute(query).unwrap();
}

pub fn populate(){
    let names = &["Alice", "Beatrix", "Charlotte", "Diana", "Emily"];
    let surnames = &["One", "Two", "Three", "Four", "Five"];
    let conn = sqlite::open(":lectager.db:").unwrap();
    for pair in names.iter().zip(surnames.iter()){
        let query = format!("INSERT OR IGNORE INTO Student (id, name, surname) VALUES ('{}{}', '{}', '{}')", pair.0, pair.1, pair.0, pair.1);
        conn.execute(query).unwrap();
    }
}