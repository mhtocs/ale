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
                987654321,
                42
            ); 
        "#,
        &pool,
    )
    .await;

    let fetch = run_query("SELECT * from INDEX_METRICS", &pool).await;

    dbg!(fetch);
}

//util method
async fn run_query(query: &str, pool: &SqlitePool) {
    sqlx::query(query).execute(pool).await.unwrap();
}
