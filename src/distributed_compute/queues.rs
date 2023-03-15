use std::sync::Mutex;
use crate::neat::trainer::{fitness::novelty_component_store::NoveltyComponentStore, configuration::Configuration};
use super::models::{to_agent_work_item::ToAgentWorkItem, from_agent_work_result::FromAgentWorkResult};


lazy_static! {
    pub static ref CPU_DISTRIBUTED_WORK_QUEUE: Mutex<CpuDistibutedWorkQueue> =  Mutex::new(CpuDistibutedWorkQueue::new());
    pub static ref CPU_DISTRIBUTED_RESULTS_QUEUE: Mutex<CpuDistibutedResultsQueue> =  Mutex::new(CpuDistibutedResultsQueue::new());
}

pub struct CpuDistibutedWorkQueue{
    pub to_agent_queue: Vec<ToAgentWorkItem>,
    pub latest_novelty_component_store: Option<NoveltyComponentStore>,
    pub latest_configuration: Option<Configuration>,
    pub current_generation: u32
}
impl CpuDistibutedWorkQueue{
    pub fn new() -> Self{
        Self{
            latest_novelty_component_store: None,
            latest_configuration: None,
            to_agent_queue: Vec::new(),
            current_generation: 0
        }
    }
}

pub struct CpuDistibutedResultsQueue{
    pub from_agent_results_queue: Vec<FromAgentWorkResult>,
}
impl CpuDistibutedResultsQueue{
    pub fn new() -> Self{
        Self{
            from_agent_results_queue: Vec::new()
        }
    }
}