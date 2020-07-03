use log::debug;
mod app;
mod config;
mod handlers;
mod models;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    pretty_env_logger::init();
    dotenv::dotenv().ok();

    let opt = config::Opt::from_args();
    let (host, port) = (opt.host.to_string(), opt.port);
    debug!("host:: {}, port:: {}", host, port);

    let app = app::make_app(opt);
    app.listen(format!("{}:{}", host, port)).await
}
