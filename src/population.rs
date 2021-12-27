use rand::prelude::SmallRng;

use crate::{cfg::Config, schedule::Schedule};

#[derive(Debug)]
pub(crate) struct Population<'c> {
    conf: &'c Config,
    population: Vec<Schedule<'c>>,
    rng: SmallRng,
}

impl<'c> Population<'c> {
    pub(crate) fn new(conf: &'c Config, mut rng: SmallRng) -> Self {
        let mut population = Vec::with_capacity(conf.population);
        for _ in 0..conf.population {
            population.push(Schedule::new(conf, &mut rng));
        }

        Population {
            conf,
            population,
            rng,
        }
    }
}
