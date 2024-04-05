use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserSchema {
    pub username: String,
    pub password: String,
    pub email: String,
    pub fullname: String,
    pub role: i8,
    pub avatar: String,
    pub notes: String,
    pub active: i8,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUserSchema {
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub fullname: Option<String>,
    pub role: Option<i8>,
    pub avatar: Option<String>,
    pub notes: Option<String>,
    pub active: Option<i8>,
}