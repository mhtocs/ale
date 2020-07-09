pub mod config;
pub mod models;
use crate::models::{Metric, State};

pub async fn add_metric(state: State, metric: Metric) -> Result<u64, sqlx::error::Error> {
    let pool = state.db_pool.as_ref().unwrap();
    sqlx::query!(
        "INSERT into index_metrics (epoch, value) VALUES ($1,$2);",
        metric.epoch,
        metric.value
    )
    .execute(pool)
    .await
}
