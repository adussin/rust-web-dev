
use std::fs;
pub mod dbcreation;

pub mod constants;
use crate::lectager::constants::*;
mod models{
    pub mod student;
    pub mod lecture;
}

use axum::{
    response::Html,
    Router,
    routing::get,
};
use minijinja::{
    context, Environment, Error, ErrorKind,
};
use sqlite;

use models::student::Student;

pub fn get_routes() -> Router{
    Router::new()
    .route("/", get(app))
}

fn get_env() -> Environment<'static> {
    let mut env = Environment::new();
    let template_path = std::env::current_dir().unwrap().join("src/lectager/static/template");
    //println!("{}", template_path.display());
    env.set_loader(move |name| {
        let pieces = name.split('/');
        let mut path = template_path.clone();
        for piece in pieces {
            if piece != "." && piece != ".." && !piece.contains('\\') {
                path.push(piece);
            } else {
                return Ok(None);
            }
        }

        match fs::read_to_string(path) {
            Ok(result) => Ok(Some(result)),
            Err(err) => {
                if err.kind() == std::io::ErrorKind::NotFound {
                    Ok(None)
                } else {
                    Err(
                        Error::new(ErrorKind::TemplateNotFound, "failed to load template")
                            .with_source(err),
                    )
                }
            }
        }
    });
    env
}
pub async fn app() -> Html<String>{
    
    dbcreation::create_db();
    dbcreation::populate();

    let conn = sqlite::open(DBNAME).unwrap();
    let env = get_env();
    let students: Vec<Student> = models::student::get_all(conn);
    let tmpl = env.get_template("index.html").unwrap();
    
    Html(tmpl.render(context! {
        students => students,
    }).unwrap())

}
