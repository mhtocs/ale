use ale_client::config;

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();
    config::init_logger().ok();
    let opt = config::Opt::from_args();

    ale_client::run(opt.es).await
}
