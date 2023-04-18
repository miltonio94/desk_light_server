use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use std::io;

async fn index() -> HttpResponse {
    let content = web::Bytes::from_static(include_bytes!("../../desk_light_ui_client/index.html"));
    HttpResponse::Ok().content_type("text/html").body(content)
}

async fn script() -> HttpResponse {
    let content = web::Bytes::from_static(include_bytes!("../../desk_light_ui_client/elm.js"));
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
