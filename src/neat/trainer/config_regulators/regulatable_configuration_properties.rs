use strum::EnumIter;
use serde::{Deserialize, Serialize};

use crate::{common::NeatFloat, neat::trainer::configuration::Configuration};
#[derive(Clone, Copy, Debug, EnumIter, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum RegulatableConfigurationProperties{
    SpeciationGeneticDistanceThreshold,
    SpeciationOffspringOutcomeNoveltyWeight,
    MutationConnectionWeightChangeScale,
    MutationNodeBiasChangeScale,
    CppnInputMultiplierScale,
    CrossSpeciesRepoductionScale
}

impl RegulatableConfigurationProperties{
    pub fn get_configuration_value<'a>(&self, configuration: &'a mut Configuration) -> &'a mut NeatFloat{
        match self {
            RegulatableConfigurationProperties::SpeciationGeneticDistanceThreshold => &mut configuration.speciation_genetic_distance_threshold,
            RegulatableConfigurationProperties::SpeciationOffspringOutcomeNoveltyWeight => &mut configuration.speciation_offspring_outcome_novelty_weight,
            RegulatableConfigurationProperties::MutationConnectionWeightChangeScale => &mut configuration.mutation_connection_weight_change_scale,
            RegulatableConfigurationProperties::MutationNodeBiasChangeScale => &mut configuration.mutation_node_bias_change_scale,
            RegulatableConfigurationProperties::CppnInputMultiplierScale => &mut configuration.mutation_node_cppn_input_multiplier_change_scale,
            RegulatableConfigurationProperties::CrossSpeciesRepoductionScale => &mut configuration.speciation_cross_species_reproduction_scale,
        }
    }
}