use ale_client::{
    config,
    models::{Metric, State},
};
use chrono::Local;
use log::info;
use sqlx::SqlitePool;

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();
    config::init_logger().ok();
    let opt = config::Opt::from_args();

    let db_url = std::env::var("DATABASE_URL").unwrap();
    let state = State {
        db_pool: Some(SqlitePool::new(&db_url).await.unwrap()),
    };

    ale_client::run(state, opt.es).await
}
