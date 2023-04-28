use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use std::{
    env,
    fs::File,
    io::{self, Read},
    path::Path,
};

fn get_path_to_ui_dir() -> String {
    match env::var("DESK_LIGHT_UI_CLIENT") {
        Ok(p) => p,
        Err(_) => String::new(),
    }
}

fn get_file_content(path: &Path) -> String {
    let mut content = String::new();
    match File::open(path) {
        Ok(mut html) => {
            html.read_to_string(&mut content);
            ()
        }
        Err(error) => {
            content = error.to_string();
            ()
        }
    };

    content
}

async fn index() -> HttpResponse {
    let path = get_path_to_ui_dir() + "/index.html";
    let path = Path::new(&path);
    let content = get_file_content(path);

    HttpResponse::Ok().content_type("text/html").body(content)
}

async fn script() -> HttpResponse {
    let path = get_path_to_ui_dir() + "/elm.js";
    let path = Path::new(&path);
    let content = get_file_content(path);

    HttpResponse::Ok()
        .content_type("application/javascript")
        .body(content)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(actix_files::Files::new("/static", "static"))
            .route("/", web::get().to(index))
            .route("/elm.js", web::get().to(script))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
