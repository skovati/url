mod db;
mod models;

use std::error::Error;
use std::env;

use warp::{Filter, http::Response};
use crate::models::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let base_url = env::var("BASE_URL")
        .unwrap_or("http://localhost".to_string()).clone();
    let port = match env::var("IGNORE_CASE") {
        Ok(p) => p.parse::<u16>().unwrap_or(8080),
        Err(_) => 8080
    };

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
        .map(move |req: Request| {
            match db::put(req.url) {
                Ok(id) => {
                    format!("{}/{}", base_url, id)
                },
                Err(e) => {
                    format!("Error creating shorted URL: {:?}", e)
                }
            }
        });

    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("www/index.html"));

    let deps = warp::get()
        .and(warp::path::end())
        .and(warp::fs::dir("www"));

    let routes = index
        .or(deps)
        .or(get)
        .or(shorten);

    warp::serve(routes)
        .run(([0, 0, 0, 0], port))
        .await;
    Ok(())
}
