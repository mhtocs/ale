pub mod config;
pub mod sysutil;

use attohttpc::Response;
use chrono::Local;
use config::Config;
use futures::join;
use log::{debug, error, info};
use serde_json::Value;
use std::thread;
use std::time::Duration;
use sysinfo::{System, SystemExt};
use sysutil::{SystemInfo, SystemUtil};

type Result<T> = std::result::Result<T, attohttpc::Error>;

pub async fn run(cfg: Config) {
    let sys = &mut SystemUtil::with(System::new_all(), cfg.procs, cfg.max_retry);
    let mut last_timestamp = Local::now().timestamp() as f64;
    let mut last_index_total = 0;
    loop {
        let (sysinfo, indexing_rate) = join!(
            sys.get_sys_info(),
            get_index_stats(&cfg.es_url, &mut last_timestamp, &mut last_index_total,)
        );
        debug!("\nSYSINFO :: {:#?}", sysinfo);
        debug!("\nESINFO :: {:#?}", indexing_rate);
        match indexing_rate {
            Ok(rate) => info!("INDEXING RATE: {} docs/s", rate),
            Err(e) => error!("Encountered error while fetching INDEXING RATE {}", e),
        }
        let resp = post(sysinfo, &cfg.server_url);
        match resp {
            Ok(res) => match res.status() {
                r if r.is_success() => info!("UPLOAD: OK  - response: {}", r),
                _ => error!("UPLOAD: FAIL - response: {:#?}", &res),
            },
            Err(e) => error!("Encountered error while uploading metrics - error: {}", e),
        }
        debug!("sleeping for :: {}s", cfg.sleep_delay);
        thread::sleep(Duration::from_secs(cfg.sleep_delay));
        debug!("woke up");
    }
}

pub fn post(info: &SystemInfo, url: &str) -> Result<Response> {
    let resp = attohttpc::post(url).json(info)?.send()?;
    Ok(resp)
}

pub async fn get_index_stats(
    es_url: &str,
    last_timestamp: &mut f64,
    last_index_total: &mut i64,
) -> Result<f64> {
    let resp = attohttpc::get(es_url.to_owned() + "/_stats").send()?;
    let resp: Value = resp.json()?;

    let index_total = resp["_all"]["total"]["indexing"]["index_total"]
        .as_i64()
        .unwrap();
    let timestamp = Local::now().timestamp() as f64;
    let indexing_rate = (index_total - *last_index_total) as f64 / (timestamp - *last_timestamp);
    *last_timestamp = timestamp;
    *last_index_total = index_total;
    Ok(indexing_rate)
}

/*FOR USING IN SERVER SIDE
 *
pub async fn update_db(state: &State, info: &SystemInfo) -> Result<u64, sqlx::error::Error> {
    let pool = state.db_pool.as_ref().unwrap();
    pool.acquire().await?;
    sqlx::query!(
        r#"INSERT into system_metrics
        (last_updated, total_memory, used_memory, total_swap, used_swap, cpu_usage)
        VALUES ($1, $2, $3, $4, $5, $6);"#,
        info.last_updated,
        info.total_memory,
        info.used_memory,
        info.total_swap,
        info.used_swap,
        info.cpu_usage,
    )
    .execute(pool)
    .await
}
*/
