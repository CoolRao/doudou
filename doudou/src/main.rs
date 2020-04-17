use actix_web::{web, guard, App, HttpRequest, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {}!", app_name) // <- response with app_name
}

struct AppState {
    app_name: String
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(AppState {
                app_name: String::from("Actix-web"),                //传状态的操作
            })
            .service(web::scope("/api/v1")
                //.guard(guard::Header("Host", "127.0.0.1"))
                .route("/", web::get().to(index)))
            .service(web::scope("/api/v2")
                // .guard(guard::Header("Host", "users.rust-lang.org"))
                .route("/", web::get().to(index)))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}






