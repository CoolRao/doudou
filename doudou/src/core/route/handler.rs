use actix_web::Responder;

pub async fn hello_world() -> impl Responder {
    "Hello world!"
}

pub async fn index() ->impl Responder{
    "welcome doudou ...."
}

