

mod dbcreation;
mod models{
    pub mod student;
}

use axum::{
    response::Html,
};
use minijinja::{
    context, Environment,
};
use sqlite;

use models::student::Student;

pub fn app(env: Environment<'_>) -> Html<String>{

    dbcreation::create_db();
    dbcreation::populate();

    let conn = sqlite::open(":lectager.db:").unwrap();

    let students: Vec<Student> = models::student::get_all(conn);
    let tmpl = env.get_template("lectager/index.html").unwrap();
    
    Html(tmpl.render(context! {
        students => students,
    }).unwrap())

}
