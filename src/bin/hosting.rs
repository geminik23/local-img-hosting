#[macro_use]
extern crate log;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let _ = dotenv::dotenv().ok();
    env_logger::init();

    tide::log::start();
    let mut app = tide::new();

    let path = std::env::var("IMAGEHOSTING_PATH").expect("no env var IMAGEHOSTING_PATH");

    app.at("/images/*").serve_dir(path)?;

    app.listen("0.0.0.0:8000").await?;
    Ok(())
}
