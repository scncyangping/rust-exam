//! Scheduler for sync and async cron jobs.
//!
//! # Author
//!
//! - Yapi
//!
//! # Date
//!
//! 2024/05/16

use std::{str::FromStr, time::Duration};

use chrono::Local;
use cron::Schedule;
use crate::{BoxFuture, Map, Uuid};


//use crate::{datetime::DateTime, BoxFuture, Map, Uuid};

use crate::datetime::DateTime;

use super::AsyncScheduler;

pub type AsyncCronJob =
    for<'a> fn(id: Uuid, data: &'a mut Map, last_tick: DateTime) -> BoxFuture<'a>;

pub struct AsyncJob {
    /// Job Id
    id: Uuid,
    /// 基础数据
    data: Map,
    /// 是否禁止
    disabled: bool,
    /// 是否立即执行
    immediate: bool,
    /// 对应调度cron
    schedule: Schedule,
    /// 执行逻辑
    run: AsyncCronJob,
    /// 上次执行时间
    last_tick: Option<DateTime>,
}

impl AsyncJob {
    pub fn new(cron: &str, exec: AsyncCronJob) -> Self {
        let schedule = Schedule::from_str(cron)
            .unwrap_or_else(|err| panic!("invalid cron expression `{cron}`: {err}"));
        Self {
            id: Uuid::now_v7(),
            data: Map::new(),
            disabled: false,
            immediate: false,
            schedule,
            run: exec,
            last_tick: None,
        }
    }

    pub fn disable(mut self, diabled: bool) -> Self {
        self.disabled = diabled;
        self
    }

    pub fn immediate(mut self, immediate: bool) -> Self {
        self.immediate = immediate;
        self
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn data(&self) -> &Map {
        &self.data
    }

    /// Returns a mutable reference to the job data.
    pub fn data_mut(&mut self) -> &mut Map {
        &mut self.data
    }

    /// Returns `true` if the job is disabled.
    pub fn is_disabled(&self) -> bool {
        self.disabled
    }

    /// Returns `true` if the job is is executed immediately.
    pub fn is_immediate(&self) -> bool {
        self.immediate
    }

    /// Pauses the job by setting the `disabled` flag to `true`.
    pub fn pause(&mut self) {
        self.disabled = true;
    }

    /// Resumes the job by setting the `disabled` flag to `false`.
    pub fn resume(&mut self) {
        self.disabled = false;
    }

    pub fn set_last_tick(&mut self, last_tick: Option<DateTime>) {
        self.last_tick = last_tick
    }

    pub async fn tick(&mut self) {
        let now = Local::now();
        let run = self.run;
        if let Some(tict) = self.last_tick {
            for event in self.schedule.after(&tict) {
                if event > now {
                    break;
                }
                if !self.disabled {
                    run(self.id, &mut self.data, tict).await
                }
            }
        } else if self.immediate && !self.disabled {
            run(self.id, &mut self.data, now.into()).await
        }
    }

    pub async fn execute(&mut self) {
        let now = Local::now();
        let run = self.run;
        run(self.id, &mut self.data, now.into()).await;
        self.last_tick = Some(now.into());
    }
}

pub struct AsyncJobScheduler {
    jobs: Vec<AsyncJob>,
}

impl AsyncJobScheduler {
    pub fn new() -> Self {
        Self { jobs: Vec::new() }
    }
    pub fn add(&mut self, job: AsyncJob) -> Uuid {
        let job_id = job.id;
        self.jobs.push(job);
        job_id
    }
    pub fn remove(&mut self, id: Uuid) -> bool {
        self.jobs
            .iter()
            .position(|job| job.id == id)
            .map_or(false, |d| {
                self.jobs.remove(d);
                true
            })
    }

    pub fn get(&self, job_id: Uuid) -> Option<&AsyncJob> {
        self.jobs.iter().find(|j| j.id == job_id)
    }

    pub fn get_mut(&mut self, job_id: Uuid) -> Option<&mut AsyncJob> {
        self.jobs.iter_mut().find(|jo| jo.id == job_id)
    }

    pub fn time_till_next_job(&self) -> Duration {
        if self.jobs.is_empty() {
            Duration::from_millis(500)
        } else {
            let mut duration = chrono::Duration::zero();
            let now = Local::now();
            // 遍历任务池中所有任务
            for job in self.jobs.iter() {
                // 获取最近执行的一次任务时间
                for d in job.schedule.after(&now).take(1) {
                    // 计算所有任务中,最近一次执行的时间
                    let interval = d - now;
                    if duration.is_zero() || interval < duration {
                        duration = interval
                    }
                }
            }
            duration
                .to_std()
                .unwrap_or_else(|_| Duration::from_millis(500))
        }
    }

    pub async fn tick(&mut self) {
        for job in self.jobs.iter_mut() {
            job.tick().await
        }
    }

    pub async fn execute(&mut self) {
        for job in self.jobs.iter_mut() {
            job.execute().await
        }
    }
}

impl AsyncScheduler for AsyncJobScheduler {
    fn is_ready(&self) -> bool {
        !self.jobs.is_empty()
    }

    fn time_till_next_job(&self) -> Duration {
        self.time_till_next_job()
    }

    async fn tick(&mut self) {
        self.tick().await
    }

    // fn tick(&mut self) -> Pin<Box<dyn Future<Output = ()> + Send>> {
    //     Box::pin(async move {
    //         self.tick().await;
    //     })
    // }
}
