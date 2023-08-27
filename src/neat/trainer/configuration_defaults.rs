use crate::{activation_functions::ActivationFunction, common::NeatFloat, neat::genome::neat::mutation_add_mode::MutationNodeAddMode};

use super::{configuration::{Configuration, OffSpringMode}, config_regulators::{config_regulator::ConfigRegulator, regulatable_configuration_properties::RegulatableConfigurationProperties, available_regulation_signals::AvailableRegulationSignals}};

pub struct ConfigurationDefaults;

impl ConfigurationDefaults{
    pub fn get() -> Configuration{
        Configuration{
            node_genes: Box::new(vec![]),
            genome_minimal_genes_to_connect_ratio: 1.0,
            mutation_node_add_probability: 0.1,
            mutation_node_delete_probability: 0.05,
            mutation_node_add_mode: MutationNodeAddMode::DontDeleteExisting,
            mutation_connection_add_probability: 0.4,
            mutation_connection_delete_probability: 0.05,
            mutation_connection_weight_change_probability: 0.3,
            mutation_connection_weight_change_scale: 0.1,
            mutation_connection_allow_recurrent: false,
            mutation_connection_weight_replace_probability: 0.005,
            mutation_node_bias_change_probability: 0.1,
            mutation_node_bias_change_scale: 0.1,
            mutation_node_cppn_input_multiplier_change_probability: 0.2,
            mutation_node_cppn_input_multiplier_change_scale: 0.2,
            mutation_node_cppn_input_multiplier_replace_probability: 0.1,
            mutation_remove_unconnected_nodes: true,
            mutation_connection_disable_probability: 0.0,
            mutation_node_available_activation_functions: ActivationFunction::RELU | ActivationFunction::SIGMOID | ActivationFunction::TANH | ActivationFunction::BINARY,
            connection_weight_min_value: -1.0,
            connection_weight_max_value: 1.0,
            node_bias_min_value: -1.0,
            node_bias_max_value: 1.0,
            population_size: 1000,
            survival_threshold: 0.2,
            target_species: 20,
            speciation_genetic_distance_threshold: 5.0,
            speciation_drop_species_no_improvement_generations: 10,
            speciation_add_new_species_during_run: true,
            speciation_remove_stagnant_species_generations: 100,
            speciation_offspring_mode: OffSpringMode::AdjustedSpeciesRange,
            speciation_offspring_outcome_novelty_weight: 0.1,
            speciation_new_species_protected_for_generations: 5,
            speciation_use_best_seed_bank: Some(5),
            speciation_add_best_member_back_in: true,
            speciation_preserve_elite: true,
            speciation_max_threshold: 4.0,
            speciation_min_threshold: 0.1,
            speciation_species_min_number_of_members: 0,
            speciation_cross_species_reproduction_scale: 0.01,
            reproduction_weights_from_fitter_probability: 0.51,
            print_summary_interval: None,
            print_summary_number_of_species_to_show: 10,
            success_threshold: 0.0,
            event_subscriptions: vec![],
            run_save_directory: "\\neatlib\\runs\\".to_string(),
            run_name: "none".to_string()
        }
    }
    pub fn get_default_regulators(configuration: &Configuration) -> Vec<ConfigRegulator>{
        vec![ConfigRegulator {
            start_generation: 0,
            max_value_of_property: configuration.speciation_max_threshold,
            min_value_of_property: configuration.speciation_min_threshold,
            when_signal_above_change_factor: 0.5,
            when_signal_below_change_factor: -0.3,
            property_to_change: RegulatableConfigurationProperties::SpeciationGeneticDistanceThreshold,
            signal_name: AvailableRegulationSignals::NumberOfSpecies,
            signal_target: configuration.target_species as NeatFloat
        },
        ConfigRegulator {
            start_generation: 0,
            max_value_of_property: 0.8,
            min_value_of_property: 0.2,
            when_signal_above_change_factor: -0.8,
            when_signal_below_change_factor: 0.2,
            property_to_change: RegulatableConfigurationProperties::SpeciationOffspringOutcomeNoveltyWeight,
            signal_name: AvailableRegulationSignals::MaxSpeciesFitnessImprovementInLast10GensComparedToLast100GensAsFactor,
            signal_target: 0.1 as NeatFloat
        },
        ConfigRegulator {
            start_generation: 0,
            max_value_of_property: 0.8,
            min_value_of_property: 0.2,
            when_signal_above_change_factor: 0.0,
            when_signal_below_change_factor: -0.4,
            property_to_change: RegulatableConfigurationProperties::SpeciationOffspringOutcomeNoveltyWeight,
            signal_name: AvailableRegulationSignals::Oscillator10Gen,
            signal_target: 0.5 as NeatFloat
        },
        ConfigRegulator {
            start_generation: 0,
            max_value_of_property: 0.5,
            min_value_of_property: 0.01,
            when_signal_above_change_factor: 0.4,
            when_signal_below_change_factor: -0.5,
            property_to_change: RegulatableConfigurationProperties::MutationConnectionWeightChangeScale,
            signal_name: AvailableRegulationSignals::AvgSpeciesFitnessImprovementInLast10GensComparedToLast100GensAsFactor,
            signal_target: 0.01 as NeatFloat
        },
        ConfigRegulator {
            start_generation: 0,
            max_value_of_property: 0.5,
            min_value_of_property: 0.01,
            when_signal_above_change_factor: 0.4,
            when_signal_below_change_factor: -0.5,
            property_to_change: RegulatableConfigurationProperties::MutationNodeBiasChangeScale,
            signal_name: AvailableRegulationSignals::AvgSpeciesFitnessImprovementInLast10GensComparedToLast100GensAsFactor,
            signal_target: 0.01 as NeatFloat
        },
        ConfigRegulator {
            start_generation: 0,
            max_value_of_property: 20.0,
            min_value_of_property: 0.01,
            when_signal_above_change_factor: 0.4,
            when_signal_below_change_factor: -0.5,
            property_to_change: RegulatableConfigurationProperties::CppnInputMultiplierScale,
            signal_name: AvailableRegulationSignals::AvgSpeciesFitnessImprovementInLast10GensComparedToLast100GensAsFactor,
            signal_target: 0.01 as NeatFloat
        },
        ConfigRegulator {
            start_generation: 150,
            max_value_of_property: 0.06,
            min_value_of_property: 0.01,
            when_signal_above_change_factor: -0.1,
            when_signal_below_change_factor: 0.1,
            property_to_change: RegulatableConfigurationProperties::CrossSpeciesRepoductionScale,
            signal_name: AvailableRegulationSignals::AvgSpeciesFitnessImprovementInLast10GensComparedToLast100GensAsFactor,
            signal_target: 0.1 as NeatFloat
        }
        ]
    }
}