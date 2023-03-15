use hashbrown::HashMap;
use crate::{neat::population::{self, speciation::species_metric::SpeciesMetric}, common::NeatFloat};

#[derive(Debug)]
pub struct Species{
    pub id: uuid::Uuid,
    pub created_generation: u32,
    pub allowed_number_of_offspring_based_on_objective_fitness: NeatFloat,
    pub allowed_number_of_offspring_based_on_outcome_novelty: NeatFloat,
    pub species_protected_until_generation: u32,
    pub color: bevy::prelude::Color,
    pub objective_fitness: SpeciesMetric,
    pub outcome_novelty: SpeciesMetric,
    pub no_of_members: usize
}

impl Species{
    pub fn new(species: &population::speciation::species::Species) -> Self {
        Self{
            id: species.id,
            color: species.primary.genome.color,
            created_generation: species.created_generation,
            allowed_number_of_offspring_based_on_objective_fitness: species.allowed_number_of_offspring_based_on_objective_fitness,
            allowed_number_of_offspring_based_on_outcome_novelty: species.allowed_number_of_offspring_based_on_outcome_novelty,
            species_protected_until_generation: species.species_protected_until_generation,
            objective_fitness: species.objective_fitness.clone(),
            outcome_novelty: species.outcome_novelty.clone(),
            no_of_members: species.members.len()
        }
    }
}

#[derive(Debug)]
pub struct SpeciesList{
    pub species_models: Vec<Species>
}
impl SpeciesList{
    pub fn new(species_index: &HashMap<uuid::Uuid, population::speciation::species::Species>) -> Self {
        let species_models = species_index.values().map(Species::new).collect::<Vec<Species>>();
        Self{
            species_models
        }
    }
}