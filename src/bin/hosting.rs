#[macro_use]
extern crate log;

use async_std::{fs::File, io::copy};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;
use tide::prelude::*;
use tide::{Body, Request, Response};
//
// async fn req_upload(request: Request<_>) {
//
// }
//

#[derive(Clone)]
struct State {
    abs_path: Arc<String>,
}

impl State {
    fn new(path: String) -> Self {
        Self {
            abs_path: Arc::new(path),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    path: String,
    link: String,
    filename: String,
}

#[derive(Serialize, Debug)]
struct ResTask {
    result: String,
    path: String,
}

async fn req_v2_hosting(mut req: Request<State>) -> tide::Result {
    let Task {
        path,
        link,
        filename,
    } = req.body_json().await?;
    info!("received req : {}, {}, {}", path, link, filename);

    let state = req.state();

    // create folder
    let img_path = state.abs_path.as_str();
    let img_path = Path::new(img_path);
    let target_path = img_path.join(&path);
    // info!("target path {:?}", target_path);
    async_std::fs::create_dir_all(&target_path).await?;

    let target_path = target_path.join(&filename);
    info!("requested - download path {:?}", target_path);

    // download
    let mut s = false;
    let res = surf::get(&link).await;
    if let Ok(mut res) = res {
        let mut dest = File::create(target_path).await?;
        let content = res.body_bytes().await.unwrap();
        let mut content = &content[..];

        s = copy(&mut content, &mut dest).await.is_ok();
    }

    let new_path = Path::new("/images").join(&path).join(&filename);
    let new_path = new_path.to_str();

    let res = json!(ResTask {
        result: if s {
            String::from("ok")
        } else {
            String::from("error")
        },
        path: if s {
            String::from(new_path.unwrap())
        } else {
            String::from("")
        },
    });

    info!("returning result {:?}", res);
    Ok(res.into())
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let _ = dotenv::dotenv().ok();
    env_logger::init();
    let img_path = std::env::var("IMAGEHOSTING_PATH").expect("no env var IMAGEHOSTING_PATH");

    let mut app = tide::with_state(State::new(img_path.clone()));

    app.at("/images").serve_dir(img_path)?;
    app.at("/v2/hosting").post(req_v2_hosting);

    app.listen("0.0.0.0:8000").await?;
    Ok(())
}
