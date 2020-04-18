use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginForm {
    pub  username: String,
    pub  password: String,
}

#[derive(Deserialize)]
pub struct MsgForm {
    pub  title: String,
    pub  content: String,
    pub contact: String,
}

#[derive(Deserialize)]
pub struct PageForm {
    pub  page: i64,
    pub  size: i64,
}