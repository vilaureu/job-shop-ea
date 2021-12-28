use std::fmt;

use rand::{
    prelude::{Distribution, SliceRandom},
    Rng,
};

use crate::cfg::Config;

#[derive(Debug, Clone)]
pub(crate) struct Schedule<'c> {
    conf: &'c Config,
    schedule: Vec<usize>,
}

impl<'c> Schedule<'c> {
    pub(crate) fn new(conf: &'c Config, mut rng: impl Rng) -> Self {
        let mut schedule = conf.ordered_schedule.clone();
        schedule.shuffle(&mut rng);

        let s = Self { conf, schedule };
        s.verify();
        s
    }

    pub(crate) fn crossover(&self, other: &Self, mut rng: impl Rng) -> Self {
        let mut schedule = Vec::with_capacity(self.schedule.len());
        let cut = rng.gen_range(0..=self.schedule.len());

        let mut left: Vec<_> = self.conf.jobs.iter().map(|j| j.len()).collect();
        for i in 0..cut {
            let job = self.schedule[i];
            schedule.push(job);
            left[job] -= 1;
        }
        for i in 0..self.schedule.len() {
            let job = other.schedule[i];
            if left[job] == 0 {
                continue;
            }

            schedule.push(job);
            left[job] -= 1;
        }
        debug_assert_eq!(left.iter().sum::<usize>(), 0);

        let child = Self {
            conf: self.conf,
            schedule,
        };
        child.verify();
        child
    }

    pub(crate) fn mutate(&mut self, mut rng: impl Rng) {
        if !self.conf.mutation_dist.sample(&mut rng) {
            return;
        }

        let op_a = rng.gen_range(0..self.schedule.len());
        let op_b = rng.gen_range(0..self.schedule.len());
        self.schedule.swap(op_a, op_b);
        self.verify();
    }

    pub(crate) fn evaluate(&self) -> u64 {
        let mut time_max = 0;
        self.visit_schedule(|job, op_idx, time| {
            let operation = &self.conf.jobs[job][op_idx];
            time_max = time_max.max(time + operation.duration);
        });

        assert_ne!(0, time_max);
        time_max
    }

    fn visit_schedule(&self, mut f: impl FnMut(usize, usize, u64)) {
        let mut job_indices = vec![0; self.conf.jobs.len()];
        let mut times_job = vec![0u64; self.conf.jobs.len()];
        let mut times_mach = vec![0u64; self.conf.machines];

        for &job in &self.schedule {
            let op_idx = job_indices[job];
            let operation = &self.conf.jobs[job][op_idx];

            let time_job = &mut times_job[job];
            let time_mach = &mut times_mach[operation.machine];
            let time = *time_job.max(time_mach);
            f(job, op_idx, time);

            let time = time + operation.duration;
            *time_mach = time;
            *time_job = time;
            job_indices[job] += 1;
        }
    }

    #[cfg(debug_assertions)]
    fn verify(&self) {
        let mut sorted = self.schedule.clone();
        sorted.sort();
        assert_eq!(self.conf.ordered_schedule, sorted);
    }

    #[cfg(not(debug_assertions))]
    fn verify(&self) {}
}

impl<'c> fmt::Display for Schedule<'c> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut machines = vec![vec![]; self.conf.machines];

        self.visit_schedule(|job, op_idx, time| {
            let operation = &self.conf.jobs[job][op_idx];

            machines[operation.machine].push((job, op_idx, time));
        });

        for (i, machine) in machines.iter().enumerate() {
            if i != 0 {
                writeln!(f)?;
            }
            write!(f, "M{}:", i)?;
            for (job, operation, time) in machine {
                write!(f, " {}/{}@{}", job, operation, time)?;
            }
        }

        Ok(())
    }
}
