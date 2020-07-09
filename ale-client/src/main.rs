use ale_client::{
    config,
    models::{Metric, State},
};
use chrono::Local;
use log::info;
use sqlx::SqlitePool;

#[async_std::main]
async fn main() {
    pretty_env_logger::init();
    dotenv::dotenv().ok();
    let opt = config::Opt::from_args();

    let db_url = std::env::var("DATABASE_URL").unwrap();
    let state = State {
        db_pool: Some(SqlitePool::new(&db_url).await.unwrap()),
    };

    ale_client::add_metric(
        state,
        Metric {
            id: 0,
            epoch: Local::now().timestamp(),
            value: (Local::now().timestamp() as i32 / 1024),
        },
    )
    .await
    .unwrap();
    //info!("Hello world!")
}
