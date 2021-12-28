use std::cmp::Reverse;

use anyhow::Context;
use rand::{
    distributions::WeightedIndex,
    prelude::{Distribution, SmallRng},
    SeedableRng,
};
use rayon::{
    iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelBridge, ParallelIterator},
    slice::ParallelSliceMut,
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
        let population = (0..conf.population)
            .map(|_| SmallRng::from_rng(&mut rng).expect("seeding rng failed"))
            .par_bridge()
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

        let next_gen: Vec<_> = SwappedIter::new((0..self.conf.couples).map(|_| {
            (
                &self.population[dist.sample(&mut self.rng)],
                &self.population[dist.sample(&mut self.rng)],
                SmallRng::from_rng(&mut self.rng).expect("seeding rng failed"),
            )
        }))
        .par_bridge()
        .map(|(parent_a, parent_b, rng)| parent_a.crossover(parent_b, rng))
        .collect();
        self.population.extend(next_gen);

        Ok(())
    }

    pub(crate) fn mutate(&mut self) {
        self.population
            .iter_mut()
            .map(|i| {
                (
                    i,
                    SmallRng::from_rng(&mut self.rng).expect("seeding rng failed"),
                )
            })
            .par_bridge()
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

struct SwappedIter<I, T, X> {
    iter: I,
    last: Option<(T, T, X)>,
}

impl<I, T, X> SwappedIter<I, T, X> {
    fn new(iter: I) -> Self {
        Self { iter, last: None }
    }
}

impl<I, T, X> Iterator for SwappedIter<I, T, X>
where
    I: Iterator<Item = (T, T, X)>,
    T: Clone,
    X: Clone,
{
    type Item = (T, T, X);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((f, s, x)) = self.last.take() {
            return Some((s, f, x));
        }

        match self.iter.next() {
            Some(triple) => {
                self.last = Some(triple.clone());
                Some(triple)
            }
            None => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (inner_low, inner_heigh) = self.iter.size_hint();
        let mut low = inner_low.saturating_mul(2);
        if self.last.is_some() {
            low = low.saturating_add(1);
        }
        let heigh = inner_heigh
            .and_then(|h| h.checked_mul(2))
            .and_then(|h| match self.last {
                Some(_) => h.checked_add(1),
                None => Some(h),
            });
        (low, heigh)
    }
}
