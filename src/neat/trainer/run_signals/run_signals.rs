use serde::{Serialize, Deserialize};
use crate::{common::NeatFloat, neat::trainer::{generation_stats::GenerationStats, run_context::RunContext}};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct RunSignals{
    pub avg_objective_fitness_increase_as_fraction_in_last_10_generations: NeatFloat,
    pub avg_objective_fitness_increase_as_fraction_in_last_100_generations: NeatFloat,
    pub avg_objective_fitness_increase_as_fraction_in_last_1000_generations: NeatFloat,
    pub avg_fitness_improvement_in_last10_gens_compared_to_last100_gens_as_factor: NeatFloat,
    pub max_objective_fitness_increase_as_fraction_in_last_10_generations: NeatFloat,
    pub max_objective_fitness_increase_as_fraction_in_last_100_generations: NeatFloat,
    pub max_objective_fitness_increase_as_fraction_in_last_1000_generations: NeatFloat,
    pub max_fitness_improvement_in_last10_gens_compared_to_last100_gens_as_factor: NeatFloat,
    pub number_of_species: NeatFloat,
    pub oscillator_10_gen: NeatFloat,
}

impl RunSignals{
    pub fn new(run_context: &RunContext) -> Self{
        let generation_stats = &run_context.last_ten_thousand_generations_stats;
        let number_of_species = run_context.species_index.len() as NeatFloat;
        if generation_stats.len() == 0{
            return Self{
                number_of_species, 
                ..Default::default()
            };
        }

        let mut result = Self {
            number_of_species,
            oscillator_10_gen: ((run_context.current_generation as NeatFloat / 10.0).sin() / 2.0) + 0.5,
            ..Default::default() 
        };
        Self::set_change_as_fraction(generation_stats, &mut result.avg_objective_fitness_increase_as_fraction_in_last_10_generations, 10);
        Self::set_change_as_fraction(generation_stats, &mut result.avg_objective_fitness_increase_as_fraction_in_last_100_generations, 100);
        Self::set_change_as_fraction(generation_stats, &mut result.avg_objective_fitness_increase_as_fraction_in_last_1000_generations, 1000);
        result.avg_fitness_improvement_in_last10_gens_compared_to_last100_gens_as_factor = result.avg_objective_fitness_increase_as_fraction_in_last_10_generations / result.avg_objective_fitness_increase_as_fraction_in_last_100_generations;

        Self::set_change_as_fraction_max(generation_stats, &mut result.max_objective_fitness_increase_as_fraction_in_last_10_generations, 10);
        Self::set_change_as_fraction_max(generation_stats, &mut result.max_objective_fitness_increase_as_fraction_in_last_100_generations, 100);
        Self::set_change_as_fraction_max(generation_stats, &mut result.max_objective_fitness_increase_as_fraction_in_last_1000_generations, 1000);
        result.max_fitness_improvement_in_last10_gens_compared_to_last100_gens_as_factor = result.max_objective_fitness_increase_as_fraction_in_last_10_generations / result.max_objective_fitness_increase_as_fraction_in_last_100_generations;
        
        if result.max_fitness_improvement_in_last10_gens_compared_to_last100_gens_as_factor.is_nan(){
            result.max_fitness_improvement_in_last10_gens_compared_to_last100_gens_as_factor = 0.0; 
        }
        result
    }
    fn set_change_as_fraction(objective_fitness_history: &Vec<GenerationStats>, property: &mut NeatFloat, sampling_point_from_front: usize){
        let latest = objective_fitness_history[objective_fitness_history.len()-1].avg_positive_objective_fitness;
        if objective_fitness_history.len() >= sampling_point_from_front{
            let value = objective_fitness_history[objective_fitness_history.len() -sampling_point_from_front].avg_positive_objective_fitness;
            let change = latest - value;
            *property = 1.0 / (latest / change);
        }else{
            *property = 1.0; 
        }
    }
    fn set_change_as_fraction_max(objective_fitness_history: &Vec<GenerationStats>, property: &mut NeatFloat, sampling_point_from_front: usize){
        let latest = objective_fitness_history[objective_fitness_history.len()-1].max_objective_fitness;
        if objective_fitness_history.len() >= sampling_point_from_front{
            let value = objective_fitness_history[objective_fitness_history.len() -sampling_point_from_front].max_objective_fitness;
            let change = latest - value;
            *property = 1.0 / (latest / change);
        }else{
            *property = 1.0; 
        }
    }
}