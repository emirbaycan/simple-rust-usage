use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserSchema {
    pub username: String,
    pub password: String,
    pub email: String,
    pub fullname: String,
    pub role: i16,
    pub avatar: String,
    pub notes: String,
    pub active: i16,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUserSchema {
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub fullname: Option<String>,
    pub role: Option<i16>,
    pub avatar: Option<String>,
    pub notes: Option<String>,
    pub active: Option<i16>,
}