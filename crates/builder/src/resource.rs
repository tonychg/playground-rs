mod ingress;
mod tags;
mod workers;

pub use ingress::*;
pub use tags::*;
pub use workers::*;

// use anyhow::Result;

pub trait Target {
    type Error;

    fn check(&self) -> Result<(), Self::Error>;
}

pub trait Source<T: Target> {
    type Error;

    fn verify(&self) -> Result<(), Self::Error>;
    fn build(&mut self) -> Result<T, Self::Error>;
}

pub enum ResourceError {
    Ingress(IngressError),
}

#[derive(Clone, Debug)]
pub enum Resource {
    Ingress(Ingress),
    Tags(TagsSource),
    Workers(WorkersSource),
}

impl Resource {
    pub fn ingress(manifest: &IngressManifest, urls: &[String]) -> Self {
        Resource::Ingress(Ingress::new(manifest, urls))
    }
}
