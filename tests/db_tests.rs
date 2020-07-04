// a sample test
#[async_std::test]
async fn test_sample() {
    pretty_env_logger::init();
    let db_url = std::env::var("DATABASE_URL").unwrap();
    dbg!(db_url);
    let pool = sqlx::SqlitePool::new("sqlite::memory:").await.unwrap();
    let conn = pool.acquire().await.unwrap();
    /*    sqlx::query!("select 1")
    .select_one(&mut conn)
    .await
    .unwrap();*/
}
