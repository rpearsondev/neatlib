use crate::neat::trainer::{neat_trainer::NeatTrainer, generation_stats::GenerationStats};

use super::species::SpeciesList;

#[derive(Debug)]
pub struct RunStats{
    pub last_ten_thousand_generations_stats: Vec<GenerationStats>,
    pub number_of_species: usize,
    pub species_list: SpeciesList
}
impl RunStats {
    pub fn new(trainer: &NeatTrainer) -> Self {
        Self {  
            last_ten_thousand_generations_stats: trainer.run_context.last_ten_thousand_generations_stats.clone(),
            number_of_species: trainer.run_context.species_index.len(),
            species_list: SpeciesList::new(&trainer.run_context.species_index)
        }
    }
}