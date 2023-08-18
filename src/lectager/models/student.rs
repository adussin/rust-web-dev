use sqlite::{
    State,
    Connection,
};
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Student{
    id: String,
    name: String,
    surname: String,
}


pub fn get_all(conn: Connection) -> Vec<Student>{
    let query = "SELECT * FROM Student";
    let mut stm = conn.prepare(query).unwrap();
    let mut students: Vec<Student> = Vec::new();
    while let Ok(State::Row) = stm.next() {
        let id: String = stm.read::<String, _>("id").unwrap();
        let name: String = stm.read::<String, _>("name").unwrap();
        let surname: String = stm.read::<String, _>("surname").unwrap();
        println!("id = {}, name = {}, surname = {}", id, name, surname);
        students.push(
            Student {
                id: id,
                name: name,
                surname: surname,
            }
        )
    }

    students
}

