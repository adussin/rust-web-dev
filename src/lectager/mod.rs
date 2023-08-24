
use std::fs;
pub mod dbcreation;

pub mod constants;
use crate::lectager::constants::*;
mod models{
    pub mod student;
    pub mod lecture;
}

use axum::{
    response::{Html, IntoResponse},
    http::{header, StatusCode, HeaderMap},
    Router,
    routing::get, extract::Path,
};
use minijinja::{
    context, Environment, Error, ErrorKind,
};
use sqlite;

use models::student::Student;

pub fn get_routes() -> Router{
    Router::new()
    .route("/", get(app))
    .route("/_static/*path", get(handle_assets))
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

static STYLES_CSS: &str = include_str!("static/css/styles.css");
static FAVICON: &str = include_str!("static/favicon.svg");

async fn handle_assets(Path(path): Path<String>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    println!("{}", path.as_str());
    match  path.as_str(){
        "css/styles.css" => {
            headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
            (StatusCode::OK, headers, STYLES_CSS)
        },
        "favicon.svg" => {
            (StatusCode::OK, headers, FAVICON)
        },
        _ => (StatusCode::NOT_FOUND, headers, ""),
    }
}