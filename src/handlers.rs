use super::config::Opt;
use super::models::*;
use log::debug;
use serde_json::{json, Value};
use tide::Request;

pub async fn search(mut req: Request<Opt>) -> tide::Result<Value> {
    let req = &mut req;
    debug!(
        "request recieved at :: {} with header :: {:#?}, body:: ",
        req.url(),
        req.header_names(),
        //req.body_string().await.unwrap()
    );
    Ok(json!(vec![
        "index_metrics".to_string(),
        "cpu_metrics".to_string(),
        "mem_metrics".to_string(),
    ]))
}

pub async fn query(mut req: Request<Opt>) -> tide::Result<Value> {
    let query: Query = req.body_json().await?;
    debug!(
        "request recieved at :: {} with header :: {:#?}, with body :: {:#?}",
        req.url(),
        req.header_names(),
        query
    );
    let mut datapoints = vec![];

    for i in 0..100 {
        datapoints.push([1212131 * i, 3]);
    }
    let payload = Payload {
        target: "index_metrics".to_string(),
        datapoints,
    };
    Ok(json!(vec![payload]))
}
