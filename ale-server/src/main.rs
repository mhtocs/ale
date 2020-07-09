use log::debug;

use ale_server::{config, make_app, models::*};
use sqlx::SqlitePool;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    pretty_env_logger::init();
    dotenv::dotenv().ok();

    let opt = config::Opt::from_args();
    let (host, port) = (opt.host.to_string(), opt.port);
    debug!("host:: {}, port:: {}", host, port);

    let db_url = std::env::var("DATABASE_URL").unwrap();
    let app = make_app(State {
        db_pool: Some(SqlitePool::new(&db_url).await.unwrap()),
    });

    app.listen(format!("{}:{}", host, port)).await
}
