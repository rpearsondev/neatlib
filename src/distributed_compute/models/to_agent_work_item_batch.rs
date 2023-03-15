use serde::{Serialize, Deserialize};
use super::to_agent_work_item::ToAgentWorkItem;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToAgentWorkItemBatch{
    pub members: Vec<ToAgentWorkItem>,
    pub generation: u32
}