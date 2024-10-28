mod app;
mod config;
mod utils;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let setting = config::Settings::new().unwrap();
    utils::init_logger(setting.app.release);

    let state = config::State::new(
        &setting.database.url,
        setting.auth.access_key,
        setting.auth.access_time,
    )
    .await;

    let app = app::init_app(state);
    app.listen(setting.app.url).await?;

    Ok(())
}
