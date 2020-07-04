use log::debug;

use ale::{config, make_app, models::State};
use sqlx::{Pool, SqlitePool};

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    pretty_env_logger::init();
    dotenv::dotenv().ok();

    let opt = config::Opt::from_args();
    let (host, port) = (opt.host.to_string(), opt.port);
    debug!("host:: {}, port:: {}", host, port);

    let db_url = std::env::var("DATABASE_URL").unwrap();
    let db_pool: SqlitePool = Pool::new(&db_url).await.unwrap();
    let app = make_app(State {
        db_pool: Some(db_pool),
    });

    app.listen(format!("{}:{}", host, port)).await
}
