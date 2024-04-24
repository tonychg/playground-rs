mod resource;
mod scheduler;

use std::path::PathBuf;

pub use resource::{Resource, ResourceError};
pub use scheduler::{Scheduler, SchedulerType};

pub enum CompilerError {
    Resource(ResourceError),
}

#[derive(Default, Clone, Debug)]
pub struct Config {
    pub ingress_base_manifest: PathBuf,
    pub ingress_urls: Vec<String>,
    pub workers_list: String,
    pub workers_manifest: PathBuf,
    pub tags: Vec<PathBuf>,
    pub scheduler_type: SchedulerType,
    pub target: PathBuf,
}

#[derive(Default, Clone, Debug)]
pub struct Compiler {
    pub config: Config,
    pub scheduler: Scheduler,
    pub resources: Vec<Resource>,
}

impl Compiler {
    pub fn configure(&mut self) -> Result<(), CompilerError> {
        Ok(())
    }

    pub fn run(&self) -> Result<(), CompilerError> {
        todo!()
    }
}
