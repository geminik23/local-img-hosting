// #[macro_use]
// extern crate log;

use std::sync::Arc;
use tide::prelude::*;
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

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let _ = dotenv::dotenv().ok();
    env_logger::init();
    let img_path = std::env::var("IMAGEHOSTING_PATH").expect("no env var IMAGEHOSTING_PATH");

    let mut app = tide::with_state(State::new(img_path.clone()));

    app.at("/images").serve_dir(img_path)?;
    app.listen("0.0.0.0:8000").await?;
    Ok(())
}
