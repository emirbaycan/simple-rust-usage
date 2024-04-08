use serde::{Deserialize,Serialize};

#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}
#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Response {
    pub count: Option<i64>,
}