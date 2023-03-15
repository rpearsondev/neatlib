use std::cmp::Ordering;
use serde::{Serialize, Deserialize};
use crate::common::NeatFloat;

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct SpeciesMember{
    pub id: uuid::Uuid,
    pub objective_fitness: NeatFloat,
    pub outcome_novelty: NeatFloat,
    pub is_elite: bool,
    pub is_cross_species: bool
}
impl SpeciesMember{
    pub fn new( id: uuid::Uuid, fitness: NeatFloat, novelty: NeatFloat, is_cross_species: bool) -> Self {
        SpeciesMember{
            id,
            objective_fitness: fitness,
            outcome_novelty: novelty,
            is_elite:false,
            is_cross_species: is_cross_species
        }
    }
}
impl Ord for SpeciesMember{
    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.objective_fitness;
        let b = other.objective_fitness;

        let x  = b.partial_cmp(&a).unwrap();
        x
    }
}
impl PartialOrd for SpeciesMember {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.objective_fitness.partial_cmp(&self.objective_fitness)
    }
}
impl PartialEq for SpeciesMember {
    fn eq(&self, other: &Self) -> bool {
        (self.id) == (other.id)
    }
}
impl Eq for SpeciesMember { }