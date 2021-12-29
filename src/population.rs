use std::cmp::Reverse;

use anyhow::Context;
use rand::{distributions::WeightedIndex, prelude::Distribution};
use rand_xoshiro::Xoshiro256PlusPlus;
use rayon::{
    iter::{
        IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator,
        IntoParallelRefMutIterator, ParallelIterator,
    },
    slice::ParallelSliceMut,
};

use crate::{cfg::Config, schedule::Schedule};

pub(crate) type FastRng = Xoshiro256PlusPlus;

#[derive(Debug)]
pub(crate) struct Population<'c> {
    conf: &'c Config,
    population: Vec<Schedule<'c>>,
    rng: FastRng,
}

impl<'c> Population<'c> {
    pub(crate) fn new(conf: &'c Config, mut rng: FastRng) -> Self {
        let rngs = gen_rngs(&mut rng, conf.population);
        let population = rngs
            .into_par_iter()
            .map(|rng| Schedule::new(conf, rng))
            .collect();

        Population {
            conf,
            population,
            rng,
        }
    }

    pub(crate) fn recombine(&mut self) -> anyhow::Result<()> {
        let fitnesses: Vec<_> = self
            .population
            .par_iter()
            .map(|s| 1.0 / s.evaluate() as f32)
            .collect();
        let dist = WeightedIndex::new(fitnesses)
            .context("cannot create weighted distribution over fitnesses")?;

        let rngs = gen_rngs(&mut self.rng, self.conf.couples);
        let next_gen: Vec<_> = rngs
            .into_par_iter()
            .flat_map(|mut rng| {
                let parent_a = &self.population[dist.sample(&mut rng)];
                let parent_b = &self.population[dist.sample(&mut rng)];
                let child_a = parent_a.crossover(parent_b, rng.clone());
                let child_b = parent_b.crossover(parent_a, rng);
                [child_a, child_b]
            })
            .collect();
        self.population.extend(next_gen);

        Ok(())
    }

    pub(crate) fn mutate(&mut self) {
        let rngs = gen_rngs(&mut self.rng, self.population.len());
        self.population
            .par_iter_mut()
            .zip(rngs)
            .for_each(|(i, rng)| i.mutate(rng));
    }

    pub(crate) fn select(&mut self) -> (&Schedule<'c>, u64) {
        let mut evaluated: Vec<_> = self
            .population
            .par_iter()
            .map(|s| s.evaluate())
            .enumerate()
            .collect();
        evaluated.par_sort_unstable_by_key(|(_, e)| *e);
        evaluated.truncate(self.conf.population);
        let best_before = evaluated[0].0;
        let best_time = evaluated[0].1;
        evaluated.par_sort_unstable_by_key(|f| Reverse(f.0));

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

fn gen_rngs(rng: &mut FastRng, count: usize) -> Vec<FastRng> {
    let mut rngs = Vec::with_capacity(count);
    for _ in 0..count {
        rngs.push(rng.clone());
        rng.jump();
    }
    rngs
}
