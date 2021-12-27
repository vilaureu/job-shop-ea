mod cfg;
mod opt;
mod population;
mod schedule;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::{bail, Context};
use cfg::{Config, Operation};
use opt::Opt;
use population::Population;
use rand::{prelude::SmallRng, SeedableRng};
use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    let rng = match opt.seed {
        Some(seed) => SmallRng::seed_from_u64(seed),
        None => SmallRng::from_entropy(),
    };
    let mut conf = (&opt).into();
    parse_file(&opt.file, &mut conf)?;

    let mut population = Population::new(&conf, rng);
    population.recombine()?;

    Ok(())
}

fn parse_file(path: &Path, mut conf: &mut Config) -> anyhow::Result<()> {
    let file = File::open(path).with_context(|| format!("failed to open {:?}", path))?;
    let reader = BufReader::new(file);

    let read_failed = || format!("failed to read from {:?}", path);

    let mut lines = reader.lines();
    match lines.next() {
        Some(e) => {
            e.with_context(read_failed)?;
        }
        None => bail!("{:?} is empty", path),
    }

    let mut jobs = vec![];
    let mut machines = 0;
    let mut ordered_schedule = vec![];
    for line in lines {
        let line = line.with_context(read_failed)?;
        let mut job = vec![];

        let mut operations = line.split_whitespace();
        while let Some(machine) = operations.next() {
            let machine: usize = machine
                .parse()
                .with_context(|| format!("cannot parse '{}' as machine number", machine))?;

            let duration = operations
                .next()
                .context("missing duration in input file")?;
            let duration = duration
                .parse()
                .with_context(|| format!("cannot parse '{}' as duration", duration))?;
            if duration == 0 {
                bail!("duration cannot be zero");
            }

            job.push(Operation { duration, machine });
            machines = machines.max(machine + 1);
        }
        ordered_schedule.extend((0..job.len()).map(|_| jobs.len()));
        jobs.push(job);
    }

    if ordered_schedule.is_empty() {
        bail!("{:?} does not contain any operation", path)
    }

    conf.jobs = jobs;
    conf.machines = machines;
    conf.ordered_schedule = ordered_schedule;
    Ok(())
}
