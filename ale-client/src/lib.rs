pub mod config;
pub mod models;
pub mod sysutil;

use crate::models::{Metric, State};
use sysinfo::{System, SystemExt};
use sysutil::SystemUtil;

pub async fn add_metric(state: State, metric: Metric) -> Result<u64, sqlx::error::Error> {
    let pool = state.db_pool.as_ref().unwrap();
    let metrics = SystemUtil::from(System::new_all(), 212, 212, 313);

    sqlx::query!(
        "INSERT into index_metrics (epoch, value) VALUES ($1,$2);",
        metric.epoch,
        metric.value
    )
    .execute(pool)
    .await
}
