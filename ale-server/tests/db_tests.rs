use ale::models;
use models::Metric;
use sqlx::SqlitePool;
// test using inmemory db (easier)

#[async_std::test]
async fn test_db_works() {
    pretty_env_logger::init();
    dotenv::dotenv().ok();

    let db_url = std::env::var("TEST_DATABASE_URL").unwrap();
    let pool = SqlitePool::new(&db_url).await.unwrap(); // sqlite::memory: doesnt work! see sqlx/issues/325
    run_query(
        r#" 
            CREATE TABLE index_metrics(
                id INTEGER NOT NULL,
                epoch INTEGER NOT NULL,
                value INTEGER NOT NULL,
                PRIMARY KEY(id)
            ); 
        "#,
        &pool,
    )
    .await;

    run_query(
        r#" 
            INSERT INTO index_metrics values(
                0,
                9876,
                42
            ); 
        "#,
        &pool,
    )
    .await;

    let fetch: Vec<Metric> = sqlx::query_as!(
        models::Metric,
        r#"
            SELECT id,epoch,
                    value
            FROM index_metrics
            WHERE epoch >= $1
                AND  epoch < $2
            GROUP by epoch / $3;
            "#,
        0,
        999999,
        20
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    log::debug!("DATA FROM MEMORY DB :: {:#?}", fetch);
}

//util method
async fn run_query(query: &str, pool: &SqlitePool) {
    sqlx::query(query).execute(pool).await.unwrap();
}
