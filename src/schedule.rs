use rand::{prelude::SliceRandom, Rng};

use crate::cfg::Config;

#[derive(Debug)]
pub(crate) struct Schedule<'c> {
    conf: &'c Config,
    schedule: Vec<usize>,
}

impl<'c> Schedule<'c> {
    pub(crate) fn new(conf: &'c Config, mut rng: impl Rng) -> Self {
        let mut schedule = conf.ordered_schedule.clone();
        schedule.shuffle(&mut rng);

        Self { conf, schedule }
    }

    pub(crate) fn evaluate(&self) -> u64 {
        let mut job_indices = vec![0; self.conf.jobs.len()];
        let mut times_job = vec![0u64; self.conf.jobs.len()];
        let mut times_mach = vec![0u64; self.conf.machines];

        let mut time_max = 0;
        for &job in &self.schedule {
            let operation = &self.conf.jobs[job][job_indices[job]];

            let time_job = &mut times_job[job];
            let time_mach = &mut times_mach[operation.machine];
            let time = *time_job.max(time_mach) + operation.duration;

            *time_mach = time;
            *time_job = time;
            job_indices[job] += 1;

            time_max = time_max.max(time)
        }

        assert_ne!(0, time_max);
        time_max
    }
}
