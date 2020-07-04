use crate::models::*;
use crate::util::*;
use serde_json::{json, Value};
use tide::Request;

pub async fn search(_: Request<State>) -> tide::Result<Value> {
    Ok(json!(vec![
        "index_metrics".to_string(),
        "cpu_metrics".to_string(),
        "mem_metrics".to_string(),
    ]))
}

pub async fn query(mut req: Request<State>) -> tide::Result<Value> {
    //let state = req.state().db_pool;

    let query: Query = req.body_json().await?;
    Ok(json!(vec![fetch_index_metrics(
        query.range.from,
        query.range.to
    )]))
}
