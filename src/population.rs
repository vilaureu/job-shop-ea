use std::cmp::Reverse;

use anyhow::Context;
use rand::{
    distributions::WeightedIndex,
    prelude::{Distribution, SmallRng},
};

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

    pub(crate) fn recombine(&mut self) -> anyhow::Result<()> {
        let fitnesses = self.population.iter().map(|s| 1.0 / s.evaluate() as f32);
        let dist = WeightedIndex::new(fitnesses)
            .context("cannot create weighted distribution over fitnesses")?;

        for _ in 0..self.conf.couples {
            let parent_a = &self.population[dist.sample(&mut self.rng)];
            let parent_b = &self.population[dist.sample(&mut self.rng)];

            let child_a = parent_a.crossover(parent_b, self.rng.clone());
            let child_b = parent_b.crossover(parent_a, self.rng.clone());
            self.population.push(child_a);
            self.population.push(child_b);
        }

        Ok(())
    }

    pub(crate) fn mutate(&mut self) {
        for individual in &mut self.population {
            individual.mutate(&mut self.rng);
        }
    }

    pub(crate) fn select(&mut self) -> (&Schedule<'c>, u64) {
        let mut evaluated: Vec<_> = self
            .population
            .iter()
            .map(|s| s.evaluate())
            .enumerate()
            .collect();
        evaluated.sort_by_key(|(_, e)| *e);
        evaluated.truncate(self.conf.population);
        let best_before = evaluated[0].0;
        let best_time = evaluated[0].1;
        evaluated.sort_by_key(|f| Reverse(f.0));

        let mut curr = 0;
        let mut retained = 0;
        let mut best_after = 0;
        self.population.retain(|_| {
            let retain = match evaluated.last() {
                Some((i, _)) if *i == curr => {
                    if curr == best_before {
                        best_after = retained;
                    }
                    evaluated.pop();
                    retained += 1;
                    true
                }
                _ => false,
            };
            curr += 1;
            retain
        });
        assert_eq!(self.conf.population, self.population.len());

        (&self.population[best_after], best_time)
    }
}
