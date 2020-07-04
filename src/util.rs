use crate::models::Payload;
use chrono::{DateTime, Utc};

pub fn fetch_index_metrics(from: DateTime<Utc>, to: DateTime<Utc>) -> Payload {
    Payload {
        target: "".to_string(),
        datapoints: vec![[1, 2]],
    }
}
