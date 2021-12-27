mod cfg;
mod opt;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::{bail, Context};
use cfg::{Config, Operation};
use opt::Opt;
use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    let mut conf = (&opt).into();
    parse_file(&opt.file, &mut conf)?;

    println!("{:?}", conf);

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
        None => return Ok(()),
    }

    let mut jobs = vec![];
    let mut machines = 0;
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
        jobs.push(job);
    }

    conf.jobs = jobs;
    conf.machines = machines;
    Ok(())
}
