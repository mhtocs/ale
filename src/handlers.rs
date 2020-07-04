use crate::models::*;
use crate::util::*;
use serde_json::{json, Value};
use tide::Request;

pub async fn search(req: Request<State>) -> tide::Result<Value> {
    let mut conn = req.state().get_conn().await;
    sqlx::query!(
        r#"
        SELECT * FROM INDEX_METRICS;
        "#
    )
    .fetch_one(&mut conn)
    .await?;
    Ok(json!(vec![
        "index_metrics".to_string(),
        "cpu_metrics".to_string(),
        "mem_metrics".to_string(),
    ]))
}

pub async fn query(mut req: Request<State>) -> tide::Result<Value> {
    dotenv::dotenv().ok();

    let query: Query = req.body_json().await?;

    Ok(json!(vec![fetch_metrics(
        query.range.from,
        query.range.to,
        MetricType::Index
    )]))
}
