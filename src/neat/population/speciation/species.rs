use std::cmp::Ordering;
use serde::{Serialize, Deserialize};
use crate::{neat::{population::GenerationMember, genome::neat::NeatGenome}, common::NeatFloat};

use super::{species_member::SpeciesMember, species_metric::SpeciesMetric};

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Species{
    pub id: uuid::Uuid,
    pub primary: GenerationMember<NeatGenome>,
    pub members: Vec<SpeciesMember>,
    pub created_generation: u32,
    pub allowed_number_of_offspring_based_on_objective_fitness: NeatFloat,
    pub allowed_number_of_offspring_based_on_outcome_novelty: NeatFloat,
    pub objective_fitness: SpeciesMetric,
    pub outcome_novelty: SpeciesMetric,
    pub adjusted_average_objective_fitness_based_on_member_range: NeatFloat,
    pub adjusted_average_outcome_novelty_based_on_member_range: NeatFloat,
    pub adjusted_average_objective_fitness_based_on_species_range: NeatFloat,
    pub adjusted_average_outcome_novelty_based_on_species_range: NeatFloat,
    pub stagnant_generation_counter: u32,
    pub stagnant_objective_fitness: NeatFloat,
    pub stagnant_novelty: NeatFloat,
    pub is_stagnant: bool,
    pub species_protected_until_generation: u32
}
impl Species{
    pub fn new(
        id: uuid::Uuid, 
        primary: GenerationMember<NeatGenome>, 
        members: Vec<SpeciesMember>, 
        created_generation: u32, 
        species_protected_until_generation: u32) -> Self{
        Species { 
            id,
            primary, 
            members, 
            created_generation, 
            allowed_number_of_offspring_based_on_objective_fitness: 0.0,
            allowed_number_of_offspring_based_on_outcome_novelty: 0.0,
            objective_fitness: SpeciesMetric::new(),
            outcome_novelty: SpeciesMetric::new(),
            adjusted_average_objective_fitness_based_on_member_range: 0.0,
            adjusted_average_outcome_novelty_based_on_member_range: 0.0,
            adjusted_average_objective_fitness_based_on_species_range: 0.0,
            adjusted_average_outcome_novelty_based_on_species_range: 0.0,
            stagnant_generation_counter: 0,
            stagnant_objective_fitness: 0.0,
            stagnant_novelty: 0.0,
            is_stagnant: false,
            species_protected_until_generation: species_protected_until_generation
        }
    }
}

impl Ord for Species{
    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.adjusted_average_objective_fitness_based_on_member_range;
        let b = other.adjusted_average_objective_fitness_based_on_member_range;

        let x  = b.partial_cmp(&a).unwrap();
        x
    }
}
impl PartialOrd for Species {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.adjusted_average_objective_fitness_based_on_member_range.partial_cmp(&self.adjusted_average_objective_fitness_based_on_member_range)
    }
}
impl PartialEq for Species {
    fn eq(&self, other: &Self) -> bool {
        (self.id) == (other.id)
    }
}
impl Eq for Species { }
