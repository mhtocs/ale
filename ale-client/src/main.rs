use ale_client::config::{self, Config, Opt};

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();
    config::init_logger().ok();
    //config::set_ctrlc();

    let opt = Opt::from_args();
    let config_path = opt.config.with_default("ale-config.yml");
    let cfg: Config = confy::load_path(config_path.to_str().unwrap()).unwrap();
    log::info!("config_path:: {:#?}", config_path);
    log::info!("{:#?}", cfg);
    if cfg.procs.is_empty() {
        panic!("no procs found!")
    }

    ale_client::run(cfg.procs, cfg.max_retry, cfg.sleep_delay).await
}
