use crate::opt::Opt;

#[derive(Debug)]
pub(crate) struct Operation {
    pub(crate) duration: u64,
    pub(crate) machine: usize,
}
pub(crate) type Jobs = Vec<Vec<Operation>>;

#[derive(Default, Debug)]
pub(crate) struct Config {
    pub(crate) jobs: Jobs,
    pub(crate) machines: usize,
    pub(crate) ordered_schedule: Vec<usize>,
    pub(crate) population: usize,
}

impl From<&Opt> for Config {
    fn from(opt: &Opt) -> Self {
        Self {
            population: opt.population,
            ..Default::default()
        }
    }
}
