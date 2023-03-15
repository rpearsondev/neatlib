use serde::{Serialize, Deserialize};

use crate::{neat::trainer::run_context::RunContext, common::NeatFloat};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GenerationStats{
    pub max_objective_fitness: NeatFloat,
    pub avg_positive_objective_fitness: NeatFloat,
    pub avg_generations_since_last_objective_fitness_improvement: NeatFloat,
    pub max_outcome_novelty: NeatFloat,
    pub avg_outcome_novelty: NeatFloat,
}

impl GenerationStats{
    pub fn new(run_context: &RunContext) -> Self{
        let mut max_objective_fitness: NeatFloat = NeatFloat::MIN;
        let mut avg_positively_adjusted_objective_fitness: NeatFloat = 0.0;
        let mut avg_generations_since_last_objective_fitness_improvement: NeatFloat = 0.0;
        let mut max_outcome_novelty: NeatFloat = NeatFloat::MIN;
        let mut avg_outcome_novelty: NeatFloat = 0.0;
        
        let number_of_species = run_context.species_index.len() as NeatFloat;

        for (_, species) in &run_context.species_index{
            if species.objective_fitness.max > max_objective_fitness{
                max_objective_fitness = species.objective_fitness.max_positively_adjusted;
            }

            avg_positively_adjusted_objective_fitness += species.objective_fitness.positively_adjusted_average;

            avg_generations_since_last_objective_fitness_improvement += species.objective_fitness.last_generation_improved as NeatFloat;

            if species.outcome_novelty.max > max_outcome_novelty{
                max_outcome_novelty = species.outcome_novelty.max;
            }

            avg_outcome_novelty += species.outcome_novelty.max;
        }

        if run_context.best_member_so_far.is_some(){
            max_objective_fitness = run_context.best_member_so_far.as_ref().unwrap().genome.objective_fitness.unwrap();
        }

        Self { 
            max_objective_fitness: max_objective_fitness, 
            avg_positive_objective_fitness: avg_positively_adjusted_objective_fitness / number_of_species, 
            avg_generations_since_last_objective_fitness_improvement: avg_generations_since_last_objective_fitness_improvement / number_of_species,
            max_outcome_novelty: max_outcome_novelty, 
            avg_outcome_novelty: avg_outcome_novelty / number_of_species
        }
    }
}