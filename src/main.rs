use std::fs;

use axum::{
    response::Html,
    routing::get,
    extract::{State, Path},
    Router,
};
use minijinja::{
    context, Environment, Error, ErrorKind,
};

pub mod lectager;
use crate::lectager::lectager::app;

#[tokio::main]
async fn main() {   
    let mut env = Environment::new();
    let template_path = std::env::current_dir().unwrap().join("static/templates");
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
    
    // build our application with a single route
    let app = Router::new()
            .route("/", get(index))
            .route("/lectager", get(lectager))
            .with_state(env);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index(State(env): State<Environment<'_>>) -> Html<String> {
    let tmpl = env.get_template("index.html").unwrap();
    Html(tmpl.render(context! {
        name => "Diocane"
    }).unwrap())
}

async fn lectager(State(env): State<Environment<'_>>) -> Html<String> {
    app(env)
}
