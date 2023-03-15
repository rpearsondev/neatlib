use serde::{Serialize, Deserialize};

use crate::common::NeatFloat;

use super::number_line::ComponentNoveltyQuantizedValue;

#[derive(Serialize, Deserialize)]
#[derive(Clone, Default, Debug)]
pub struct Fitness{
    pub objective_fitness: NeatFloat,
    pub outcome_novelty: NeatFloat,
    pub outcome_novelty_quantized_values: Option<Vec<ComponentNoveltyQuantizedValue>>
}
impl Fitness{
    pub fn objective(fitness: NeatFloat) -> Self{
        Self {
            objective_fitness: fitness,
            outcome_novelty: 0.0,
            outcome_novelty_quantized_values: None
        }
    }
    pub fn average_fitness(fitness: Vec<Fitness>) -> Self{
        let x = fitness.iter().flat_map(|f| f.outcome_novelty_quantized_values.as_ref().unwrap_or(&vec![]).clone()).collect::<Vec<ComponentNoveltyQuantizedValue>>();
        Self {
            objective_fitness: fitness.iter().map(|f| f.objective_fitness).sum::<NeatFloat>() / fitness.len() as NeatFloat,
            outcome_novelty: fitness.iter().map(|f| f.outcome_novelty).sum::<NeatFloat>() / fitness.len() as NeatFloat,
            outcome_novelty_quantized_values: Some(x)
        }
    }
}
