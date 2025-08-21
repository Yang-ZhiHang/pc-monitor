use tokio::runtime::Runtime;
use tokio::time::{Duration, interval};

use std::sync::LazyLock;

pub static RT: LazyLock<Runtime> = LazyLock::new(|| Runtime::new().unwrap());

pub fn register_scheduled_task<F>(task: F, duration: Duration)
where
    F: Fn() + Send + Sync + 'static,
{
    RT.spawn(async move {
        let mut itv = interval(duration);
        loop {
            task();
            itv.tick().await;
        }
    });
}
