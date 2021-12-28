mod cfg;
mod opt;
mod population;
mod schedule;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering::Relaxed},
        Arc,
    },
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
    let mut conf = Config::from_opt(&opt)?;
    parse_file(&opt.file, &mut conf)?;

    let mut population = Population::new(&conf, rng);

    let terminate = Arc::new(AtomicBool::new(false));
    let terminate_clone = terminate.clone();
    match ctrlc::set_handler(move || terminate_clone.store(true, Relaxed)) {
        Err(e) if opt.iterations.is_none() => Err(e).context("cannot create interrupt handler")?,
        Err(_) => eprintln!("cannot create interrupt handler"),
        Ok(_) => {}
    }

    let mut best = None;
    let mut i = 0;
    while !terminate.load(Relaxed) && match opt.iterations {
        Some(iters) if i < iters => {
            i += 1;
            true
        }
        Some(_) => false,
        None => true,
    } {
        population.recombine()?;
        population.mutate();
        let curr = population.select();
        if best.as_ref().map_or(true, |(_, s)| curr.1 < *s) {
            best = Some((curr.0.clone(), curr.1));
        }
        eprintln!("Current time: {}, best: {}", curr.1, best.as_ref().unwrap().1);
    }
    eprintln!("--------------------");

    if let Some(best) = best {
        println!("Best schedule with time {}:", best.1);
        println!("{}", best.0);
    }

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
