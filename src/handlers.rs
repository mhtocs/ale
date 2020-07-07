use crate::models::*;
use serde_json::{json, Value};
use tide::Request;

pub async fn search(_req: Request<State>) -> tide::Result<Value> {
    Ok(json!(vec![
        "index_metrics".to_string(),
        "cpu_metrics".to_string(),
        "mem_metrics".to_string(),
    ]))
}

pub async fn query(mut req: Request<State>) -> tide::Result<Value> {
    let query: Query = req.body_json().await?;

    let pool = req.state().db_pool.as_ref().unwrap();

    Ok(json!(vec![Metric::fetch_metrics(
        query.range.from,
        query.range.to,
        MetricType::Index,
        pool
    )]))
}
