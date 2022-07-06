use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Request {
    pub url: String,
}

#[derive(Serialize, Debug)]
pub struct Response {
    pub id: String,
}
