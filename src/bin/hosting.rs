#[macro_use]
extern crate log;

use async_std::{
    fs::{remove_file, File},
    io::copy,
};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;
use tide::prelude::*;
use tide::Request;
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

#[derive(Serialize, Deserialize, Debug)]
struct DelTask {
    url: String,
}

#[derive(Serialize, Debug)]
struct ResTask {
    result: String,
    path: Option<String>,
}

async fn req_v2_hosting_del(mut req: Request<State>) -> tide::Result {
    let DelTask { url } = req.body_json().await?;
    let mut s = false;
    info!("REQ REMOVE : {}", url);

    let state = req.state();

    // create folder
    let img_path = state.abs_path.as_str();
    let img_path = Path::new(img_path);
    let target_path = img_path.join(&url);

    // open file
    let file = File::open(&target_path).await;
    if file.is_ok() {
        let result = remove_file(&target_path).await;
        s = result.is_ok();
        if s {
            info!("... removed {}", url);
        }
    }

    let res = json!(ResTask {
        result: if s {
            String::from("ok")
        } else {
            String::from("error")
        },
        path: None,
    });
    Ok(res.into())
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

    info!("requested - download path {:?}", target_path);
    // info!("target path {:?}", target_path);
    if let Err(err) = async_std::fs::create_dir_all(&target_path).await {
        error!("failed to create directory {:?}", err);
    }

    let target_path = target_path.join(&filename);

    // download
    let mut s = false;
    let res = surf::get(&link).await;
    match res {
        Ok(mut res) => {
            let mut dest = File::create(target_path).await?;
            let content = res.body_bytes().await.unwrap();
            let mut content = &content[..];

            s = copy(&mut content, &mut dest).await.is_ok();
        }
        Err(err) => {
            error!("failed to download {:?} {:?}", target_path, err);
        }
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
            Some(String::from(new_path.unwrap()))
        } else {
            None
        },
    });

    info!("returning result {:?}", res);
    Ok(res.into())
}

#[derive(Serialize, Debug)]
struct ResStorage {
    pub total: u64,
    pub usage: u64,
}

async fn req_storage(mut req: Request<State>) -> tide::Result {
    info!("REQ GET storage");
    let res = ResStorage {
        total: 0u64,
        usage: 0u64,
    };

    Ok(json!(res).into())
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let _ = dotenv::dotenv().ok();
    env_logger::init();
    let img_path = std::env::var("IMAGEHOSTING_PATH").expect("no env var IMAGEHOSTING_PATH");

    let mut app = tide::with_state(State::new(img_path.clone()));

    app.at("/images").serve_dir(img_path)?;
    app.at("/v2/hosting")
        .post(req_v2_hosting)
        .delete(req_v2_hosting_del);
    app.at("/v2/storage").get(req_storage);

    app.listen("0.0.0.0:8000").await?;
    Ok(())
}
