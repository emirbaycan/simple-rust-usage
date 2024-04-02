use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTestimonialSchema {
    pub name: String,
    pub comment: String,
    pub position: String,
    pub company: String,
    pub img: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTestimonialSchema {
    pub name: Option<String>,
    pub comment: Option<String>,
    pub position: Option<String>,
    pub company: Option<String>,
    pub img: Option<String>,
}