use actix_web::{web, Result, HttpRequest, HttpResponse};
mod form;

use form::{LoginForm, MsgForm};


// login
pub async fn login_handler(info: web::Json<LoginForm>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

// message
pub async fn message_handler(msg: web::Json<MsgForm>) -> Result<String> {
    Ok(format!("Welcome {}!", msg.content))
}


