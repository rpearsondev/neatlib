use serde::{Serialize, Deserialize};

use crate::neat::{genome::neat::NeatGenome, population::GenerationMember};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToAgentWorkItem{
    pub member: GenerationMember<NeatGenome>
}