use actix_web::{web, App, HttpServer};
use actix_files as fs;
use std::path::PathBuf;

mod route;

#[actix_rt::main]
pub async fn run(config: PrjConfig) -> std::io::Result<()> {
    println!("{:?}", config.mongodb_url);
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api/v1").configure(route::config))
            .service(fs::Files::new("/", "./static/").index_file("index.html"))  //首页
        //.service(fs::Files::new("/static", "./static/").show_files_listing())   // 静态资源目录
    })
        .bind(config.host)?
        .run()
        .await
}


#[derive(RustcDecodable, RustcEncodable)]
pub struct PrjConfig {
    pub mongodb_url: String,
    pub debug: bool,
    pub host: String,
}