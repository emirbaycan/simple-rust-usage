use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateDetailSchema {
    pub title: String,
    pub logo: String,
    pub keywords: String,
    pub site_description: String,
    pub description: String,
    pub about: String,
    pub position: String,
    pub company: String,
    pub img: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateDetailSchema {
    pub title: Option<String>,
    pub logo: Option<String>,
    pub keywords: Option<String>,
    pub site_description: Option<String>,
    pub description: Option<String>,
    pub about: Option<String>,
    pub position: Option<String>,
    pub company: Option<String>,
    pub img: Option<String>,
}
