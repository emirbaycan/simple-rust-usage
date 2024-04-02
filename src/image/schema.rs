use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateImageSchema {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateImageSchema {
    pub name: Option<String>,    
}
