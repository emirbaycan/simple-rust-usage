use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateDetailSchema {
    pub description: String,
    pub about: String,
    pub position: String,
    pub company: String,
    pub img: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateDetailSchema {
    pub description: Option<String>,
    pub about: Option<String>,
    pub position: Option<String>,
    pub company: Option<String>,
    pub img: Option<String>,
}
