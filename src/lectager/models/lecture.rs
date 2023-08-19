

use sqlite::{
    Connection,
};
use serde_derive::{Serialize, Deserialize};
/*use chrono::naive::{
    NaiveDate, NaiveTime,
};*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lecture{

    student_id: String,
    is_done: bool,
    hourly_rate: f64,
    is_paid: bool,
}

pub fn get_all(conn: Connection){

}

pub fn insert_all(conn: Connection){

}
