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
}
