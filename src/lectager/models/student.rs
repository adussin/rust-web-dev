use sqlite::{
    State,
    Connection,
};
use serde_derive::{Serialize, Deserialize};

use crate::lectager::models::lecture;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Student{
    id: String,
    name: String,
    surname: String,
}

impl Student {
    /*
    fn get_student_lectures(&self){
        let query = "SELECT * FROM Lecture WHERE student_id = ?";
        let mut stm = conn.prepare(query).unwrap();
        stm.bind((1, self.id)).unwrap();

        let lectures: Vec<Lectures> = Vec::new();
        while let Ok(State::Row) = stm.next() {
            let student_id: String = stm.read::<String, _>("id").unwrap();
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
    }*/
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

