pub mod config;
pub mod sysutil;

use log::info;
use std::thread;
use std::time::Duration;
use sysinfo::{System, SystemExt};
use sysutil::SystemUtil;

pub async fn run(pid: i32) {
    let sys = &mut SystemUtil::with(System::new_all(), vec![("ela", pid)]);
    loop {
        let info = sys.get_sys_info();
        info!("\n:: {:#?}", info);
        info!("flushing to db");

        info!("flushed sucessfully");
        info!("sleeping for :: 5s");
        thread::sleep(Duration::from_secs(5));
        info!("woke up");
    }
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
