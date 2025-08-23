use chrono::{Duration as ChronoDuration, Local, Timelike};
use log::debug;
use rdev::{Event, listen};
use std::sync::LazyLock;
use tokio::runtime::Runtime;
use tokio::time::{Duration, interval, sleep};

pub static RT: LazyLock<Runtime> = LazyLock::new(|| Runtime::new().unwrap());

pub fn register_scheduled_task<F>(id: &'static str, task: F, duration: Duration)
where
    F: Fn() + Send + Sync + 'static,
{
    RT.spawn(async move {
        let mut itv = interval(duration);
        loop {
            debug!("{} executed.", id);
            task();
            itv.tick().await;
        }
    });
}

pub fn register_event_listener<F>(_id: &'static str, task: F)
where
    F: FnMut(Event) + Send + Sync + 'static,
{
    listen(task).expect("Error listening for events");
}

pub fn _run_daily_task<F>(id: &'static str, task: F, hour: u32, minute: u32, second: u32)
where
    F: Fn() + Send + Sync + 'static,
{
    RT.spawn(async move {
        let target_time = Local::now()
            .date_naive()
            .and_hms_opt(hour, minute, second)
            .unwrap();
        let now = Local::now()
            .date_naive()
            .and_hms_opt(
                Local::now().hour(),
                Local::now().minute(),
                Local::now().second(),
            )
            .unwrap();

        // If the target time is in the past today, schedule for tomorrow
        let next_run = if now > target_time {
            target_time + ChronoDuration::days(1)
        } else {
            target_time
        };

        let wait_time = (next_run - now).num_seconds() as u64;
        sleep(Duration::from_secs(wait_time)).await;

        debug!("{} executed.", id);
        task();

        // Schedule the task to run every 24 hours
        let mut itv = interval(Duration::from_secs(wait_time));
        loop {
            itv.tick().await;
            debug!("{} executed.", id);
            task();
        }
    });
}

#[test]
fn test_run_daily_task() {
    let target_time = Local::now().date_naive().and_hms_opt(20, 0, 0).unwrap();
    let current_time = Local::now()
        .date_naive()
        .and_hms_opt(
            Local::now().hour(),
            Local::now().minute(),
            Local::now().second(),
        )
        .unwrap();
    debug!("Current time: {}", current_time);
    debug!("Target time: {}", target_time);
}
