mod db;
mod models;

use std::error::Error;

use warp::{Filter, http::Response};
use crate::models::Request;

const BASE_URL: &str = "http://localhost:8080/";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    db::init()?;

    let get = warp::get()
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .map(|id| {
            match db::get(id) {
                Ok(url) => {
                    Response::builder()
                        .header("Location", url)
                        .status(301)
                        .body("")
                        .unwrap()
                },
                Err(_) => {
                    Response::builder()
                        .status(404)
                        .body("")
                        .unwrap()
                }
            }
        });

    let shorten = warp::post()
        .and(warp::path::end())
        .and(warp::body::json())
        .map(|req: Request| {
            match db::put(req.url) {
                Ok(id) => {
                    format!("{}{}", BASE_URL, id)
                },
                Err(e) => {
                    format!("Error creating shorted URL: {:?}", e)
                }
            }
        });

    let routes = get
        .or(shorten);

    warp::serve(routes)
        .run(([0, 0, 0, 0], 8080))
        .await;
    Ok(())
}
