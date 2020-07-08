use crate::models::*;
use serde_json::{json, Value};
use tide::Request;

pub async fn search(_req: Request<State>) -> tide::Result<Value> {
    Ok(json!(vec!["index_metrics", "cpu_metrics", "mem_metrics",]))
}

pub async fn query(mut req: Request<State>) -> tide::Result<Value> {
    let query: Query = req.body_json().await?;
    let _type = query.targets.first().unwrap()._type.as_str();
    let _type = match _type {
        "index_metrics" => MetricType::Index,
        "cpu_metrics" => MetricType::Cpu,
        "mem_metrics" => MetricType::Mem,
        _ => MetricType::Nil,
    };

    let pool = req.state().db_pool.as_ref().unwrap();

    Ok(json!(vec![
        Metric::fetch_metrics(
            query.range.from,
            query.range.to,
            query.interval_ms,
            _type,
            pool
        )
        .await
    ]))
}
