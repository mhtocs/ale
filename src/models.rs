use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Serialize)]
pub struct Payload {
    pub target: String,
    pub datapoints: Vec<[i64; 2]>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Range {
    pub from: chrono::DateTime<Utc>,
    pub to: chrono::DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Target {
    pub target: Option<String>,

    #[serde(rename = "refId")]
    pub ref_id: String,

    #[serde(rename = "type")]
    pub _type: String,
}

//https://medium.com/@vsbabu/sqlite3-cte-tricks-for-time-series-analysis-196dbf3ffdf9

#[derive(Debug, Serialize, Deserialize)]
pub struct Query {
    pub range: Range,

    #[serde(rename = "intervalMs")]
    pub interval_ms: u64,

    #[serde(rename = "maxDataPoints")]
    pub max_data_points: u64,

    pub targets: Vec<Target>,
}

#[derive(Debug)]
pub struct State {
    pub db_pool: Option<sqlx::SqlitePool>,
}

#[derive(Debug, Serialize)]
pub enum MetricType {
    Index,
    Cpu,
    Mem,
    Nil,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metric {
    pub id: i32,
    pub epoch: i32,
    pub value: i32,
}

impl Metric {
    pub async fn fetch_metrics(
        _from: DateTime<Utc>,
        _to: DateTime<Utc>,
        _interval: u64,
        _type: MetricType,
        pool: &SqlitePool,
    ) -> Vec<Self> {
        let fetch: Vec<Metric> = sqlx::query_as!(
            Metric,
            r#"
            SELECT id,
                   epoch,
                   value
            FROM  index_metrics
            "#,
            //0,
            //999999,
        )
        .fetch_all(pool)
        .await
        .unwrap();

        match _type {
            MetricType::Index => (),
            _ => (),
        }

        log::debug!("FETCH_METRICS FROM PERSISTED DB:: {:#?}", &fetch);
        fetch
    }
}
