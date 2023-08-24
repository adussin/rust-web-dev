use std::fs;

use axum::{
    response::Html,
    routing::get,
    extract::{State},
    Router,
};
use minijinja::{
    context, Environment, Error, ErrorKind,
};

pub mod lectager;

#[tokio::main]
async fn main() {   
    
    let lectager_routes = lectager::get_routes();
    let app = Router::new()
            .route("/", get(index))
            .nest("/lectager", lectager_routes); // like django's view importation

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn get_env() -> Environment<'static> {
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

    env
}
async fn index() -> Html<String> {
    let env = get_env();
    let tmpl = env.get_template("index.html").unwrap();
    Html(tmpl.render(context! {
        name => "Diocane"
    }).unwrap())
}
