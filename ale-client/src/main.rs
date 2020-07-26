use ale_client::config::{self, Config, Opt};
use url::Url;

#[async_std::main]
async fn main() -> () {
    dotenv::dotenv().ok();

    let opt = Opt::from_args();
    let config_path = opt.config.with_default("ale-config.yml");
    let cfg: Config = confy::load_path(config_path.to_str().unwrap()).unwrap();
    config::init_logger(cfg.loglevel.as_ref()).ok();
    config::set_ctrlc();

    log::info!("config_path:: {:#?}", config_path);
    log::info!("{:#?}", cfg);
    if cfg.procs.is_empty() {
        panic!("no procs found!")
    }

    if cfg.server_url.is_empty() {
        panic!("server_url can't be empty")
    }

    if !cfg.server_url.is_empty() {
        Url::parse(cfg.server_url.as_ref()).unwrap();
    }

    ale_client::run(cfg).await
}
