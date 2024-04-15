use serde::{Deserialize,Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize, Debug, Default)]
pub struct Login {
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Default, Deserialize, Serialize)]
pub struct Logged(bool);

#[derive(Default, FromRow, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password: String,
    pub fullname: String,
    pub role: i16,
    pub avatar: String,
    pub active: i16,
}
pub struct NewUser {
    pub id: Uuid
}