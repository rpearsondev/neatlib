use crate::common::NeatFloat;

use super::{super::trainer::{configuration::Configuration, run_context::RunContext}, neat::mutation_mode::MutationMode};

pub trait Genome{
    fn get_id(&self) -> uuid::Uuid;
    fn minimal(configuration: &Configuration, run_context: &mut RunContext) -> Self;
    fn mutate(&mut self, configuration: &Configuration, run_context: &mut RunContext, mutation_mode: MutationMode);
    fn dont_allow_mutation(&mut self);
    fn set_objective_fitness(&mut self, value: NeatFloat);
    fn set_novelty(&mut self, value: NeatFloat);
    fn get_fitness(&self) -> Option<NeatFloat>;
    fn get_genetic_difference_distance_from(&mut self, other_genome: &Self, stop_when_hit: NeatFloat) -> NeatFloat;
    fn get_complexity(&self) -> NeatFloat;
}
