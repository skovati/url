use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Request {
    pub url: String,
}
