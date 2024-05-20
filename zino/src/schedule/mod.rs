use std::{future::Future, time::Duration};
pub mod async_job;
pub trait Scheduler {
    fn is_ready(&self) -> bool;

    fn time_till_next_job(&self) -> Duration;

    fn tick(&mut self);
}

/// Async任务调度器
pub trait AsyncScheduler {
    /// 返回任务调度器是否准备好执行
    fn is_ready(&self) -> bool;
    /// 返回到下一个任务运行的时间
    fn time_till_next_job(&self) -> Duration;
    /// 增加调度器的时间,并执行挂起的任务
    fn tick(&mut self) -> impl Future<Output = ()> + Send;
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use zino::{BoxFuture, Map, Uuid};

    use crate::{datetime::DateTime, schedule::AsyncScheduler};

    use super::async_job::{AsyncCronJob, AsyncJob, AsyncJobScheduler};

    #[tokio::test]
    async fn test_async_scheduler() {
        let mut schedule = add_scheduler();
        if schedule.is_ready() {
            tokio::spawn(async move {
                loop {
                    schedule.tick().await;
                    tokio::time::sleep(schedule.time_till_next_job()).await
                }
            });
        }
        println!("start...");
        tokio::time::sleep(Duration::from_secs(1000)).await;
        println!("end...");
    }

    pub fn add_scheduler() -> AsyncJobScheduler {
        let mut scheduler = AsyncJobScheduler::new();
        let job = AsyncJob::new("1 * * * * *", print_task as AsyncCronJob).immediate(true);
        scheduler.add(job);
        scheduler
    }

    pub fn print_task(job_id: Uuid, job_data: &mut Map, last_tick: DateTime) -> BoxFuture {
        job_data.insert("job_id".to_string(), job_id.to_string().into());
        job_data.insert("time".to_string(), last_tick.to_string().into());
        Box::pin(async {
            let job_id = job_data.get("job_id".into());
            let time = job_data.get("time".into());
            println!("job_id:{}, time:{}", job_id.unwrap(), time.unwrap());
        })
    }
}
