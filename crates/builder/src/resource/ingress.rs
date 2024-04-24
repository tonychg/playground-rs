use super::{Source, Target};

// use anyhow::Result;

#[derive(thiserror::Error, Clone, Debug)]
pub enum IngressError {
    #[error("Ingress Parse Error")]
    ParseError,
}

#[derive(Default, Clone, Debug)]
pub struct IngressUrl {
    pub url: String,
}

#[derive(Default, Clone, Debug)]
pub struct IngressManifest {}

#[derive(Default, Clone, Debug)]
pub struct Ingress {
    pub base: Vec<IngressManifest>,
    pub urls: Vec<IngressUrl>,
}

impl Ingress {
    pub fn new(manifest: &IngressManifest, urls: &[String]) -> Self {
        Ingress {
            base: vec![manifest.clone()],
            urls: urls
                .iter()
                .map(|url| IngressUrl { url: url.clone() })
                .collect(),
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct IngressTarget {
    manifests: Vec<IngressManifest>,
}

impl Target for IngressTarget {
    type Error = IngressError;

    fn check(&self) -> Result<(), IngressError> {
        println!("Checking Ingress");
        Ok(())
    }
}

impl Source<IngressTarget> for Ingress {
    type Error = IngressError;

    fn build(&mut self) -> Result<IngressTarget, IngressError> {
        println!("Building Ingress");
        Ok(IngressTarget::default())
    }

    fn verify(&self) -> Result<(), IngressError> {
        println!("Verifying Ingress");
        Ok(())
    }
}
