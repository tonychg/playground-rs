#[derive(Default, Clone, Debug)]
pub enum SchedulerType {
    Parallel,
    #[default]
    Sequential,
}

#[derive(Default, Clone, Debug)]
pub struct Scheduler {}
