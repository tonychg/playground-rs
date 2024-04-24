#[derive(Default, Clone, Debug)]
pub struct Worker {
    pub cmd: String,
    pub replicas: u32,
    pub mem_limit: String,
}

#[derive(Default, Clone, Debug)]
pub struct WorkerManifest {}

#[derive(Default, Clone, Debug)]
pub struct WorkersSource {
    pub list: Vec<Worker>,
    pub base: WorkerManifest,
}

#[derive(Default, Clone, Debug)]
pub struct WorkersTarget {
    pub manifests: Vec<WorkerManifest>,
}
