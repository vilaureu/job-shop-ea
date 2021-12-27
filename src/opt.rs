use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about)]
pub(crate) struct Opt {
    /// Job-shop scheduling description file
    #[structopt(name = "FILE", parse(from_os_str))]
    pub(crate) file: PathBuf,

    /// Set RNG seed (unsigned integer) [default: random]
    #[structopt(short, long)]
    pub(crate) seed: Option<u64>,

    /// Set population size
    #[structopt(short, long, default_value = "100")]
    pub(crate) population: usize,

    /// Set number of couples
    #[structopt(short, long, default_value = "40")]
    pub(crate) couples: usize,

    /// Set chance of mutation in an individual
    #[structopt(short, long, default_value = "0.2")]
    pub(crate) mutation_chance: f64,
}
