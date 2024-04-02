use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateJobSchema {
    pub company: String,
    pub title: String,
    pub date: String,
    pub description: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateJobSchema {
    pub company: Option<String>,
    pub title: Option<String>,
    pub date: Option<String>,
    pub description: Option<String>,
}