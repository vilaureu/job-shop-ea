use anyhow::{bail, Context};
use rand::distributions::Bernoulli;

use crate::opt::Opt;

#[derive(Debug)]
pub(crate) struct Operation {
    pub(crate) duration: u64,
    pub(crate) machine: usize,
}
pub(crate) type Jobs = Vec<Vec<Operation>>;

#[derive(Debug)]
pub(crate) struct Config {
    pub(crate) jobs: Jobs,
    pub(crate) machines: usize,
    pub(crate) ordered_schedule: Vec<usize>,
    pub(crate) population: usize,
    pub(crate) couples: usize,
    pub(crate) mutation_dist: Bernoulli,
}

impl Config {
    pub(crate) fn from_opt(opt: &Opt) -> anyhow::Result<Self> {
        if opt.population == 0 {
            bail!("population size must not be zero");
        }
        let mutation_dist = Bernoulli::new(opt.mutation_chance).with_context(|| {
            format!(
                "mutation change {} is not in range [0, 1]",
                opt.mutation_chance
            )
        })?;

        Ok(Self {
            jobs: vec![],
            machines: 0,
            ordered_schedule: vec![],
            population: opt.population,
            couples: opt.couples,
            mutation_dist,
        })
    }
}
