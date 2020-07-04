use crate::models::{MetricType, Payload};
use chrono::{DateTime, Utc};

pub fn fetch_metrics(from: DateTime<Utc>, to: DateTime<Utc>, _type: MetricType) -> Payload {
    match _type {
        MetricType::Index => (),
        _ => (),
    }
    Payload {
        target: "".to_string(),
        datapoints: vec![[1, 2]],
    }
}
