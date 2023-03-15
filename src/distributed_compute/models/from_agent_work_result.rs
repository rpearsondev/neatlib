use serde::{Serialize, Deserialize};

use crate::neat::trainer::fitness::Fitness;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FromAgentWorkResult{
    pub genome_id: uuid::Uuid,
    pub fitness: Fitness
}