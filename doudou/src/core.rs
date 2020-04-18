use actix_web::{web, App, HttpServer};

mod route;

#[actix_rt::main]
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api/v1").configure(route::config))
            .route("/", web::get().to(route::handler::index))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}