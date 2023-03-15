use serde::{Serialize, Deserialize};

use crate::neat::trainer::{fitness::{novelty_component_store::NoveltyComponentStore}, configuration::Configuration};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenerationData{
    pub novelty_component_store: Option<NoveltyComponentStore>,
    pub configuration: Option<Configuration>,
    pub generation: u32 
}