use actix_web::{web, App, HttpServer};
use actix_files as fs;

mod route;

#[actix_rt::main]
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api/v1").configure(route::config))
            .service(fs::Files::new("/", "./static/").index_file("index.html"))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}