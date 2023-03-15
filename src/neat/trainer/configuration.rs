#![allow(unused)]
use array_tool::vec::{Uniq, self};
use serde::{Deserialize, Serialize, Serializer};

use crate::{neat::{genome::neat::{node_gene::NodeGene, mutation_add_mode::MutationNodeAddMode}, genetable::connect_gene_table::ConnectGeneTable}, hyperneat::substrate::substrate_set::SubstrateSet, node_kind::NodeKind, activation_functions::ActivationFunction, common::{NeatFloat, event_stream::{event::EventType, listeners::listeners::Listeners, event_subscription::EventSubscription}}};

use super::{configuration_defaults::ConfigurationDefaults, node_conf::NodeConf};

pub const MUTABLE_CONFIG_PARAMS: usize = 9;
#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct Configuration{
    pub node_genes: Box<Vec<NodeGene>>,
    pub genome_minimal_genes_to_connect_ratio: NeatFloat,
    pub mutation_node_add_probability: NeatFloat,
    pub mutation_node_add_mode: MutationNodeAddMode,
    pub mutation_node_delete_probability: NeatFloat,
    pub mutation_connection_add_probability: NeatFloat,
    pub mutation_connection_delete_probability: NeatFloat,
    pub mutation_connection_weight_change_probability: NeatFloat,
    pub mutation_connection_allow_recurrent: bool,
    pub mutation_connection_weight_change_scale: NeatFloat,
    pub mutation_connection_weight_replace_probability: NeatFloat,
    pub mutation_node_bias_change_probability: NeatFloat,
    pub mutation_node_bias_change_scale: NeatFloat,
    pub mutation_node_cppn_input_multiplier_change_probability: NeatFloat,
    pub mutation_node_cppn_input_multiplier_change_scale: NeatFloat,
    pub mutation_node_cppn_input_multiplier_replace_probability: NeatFloat,
    pub mutation_remove_unconnected_nodes: bool,
    pub mutation_connection_disable_probability: NeatFloat,
    pub mutation_node_available_activation_functions: ActivationFunction,
    pub node_bias_min_value: NeatFloat,
    pub node_bias_max_value: NeatFloat,
    pub connection_weight_min_value: NeatFloat,
    pub connection_weight_max_value: NeatFloat,
    pub population_size: u32,
    pub survival_threshold: NeatFloat,
    pub target_species: u32,
    pub speciation_genetic_distance_threshold: NeatFloat,
    pub speciation_drop_species_no_improvement_generations: u32,
    pub speciation_add_new_species_during_run: bool,
    pub speciation_remove_stagnant_species_generations: u32,
    pub speciation_offspring_mode: OffSpringMode,
    pub speciation_offspring_outcome_novelty_weight: NeatFloat,
    pub speciation_new_species_protected_for_generations: u32,
    pub speciation_use_best_seed_bank: Option<usize>,
    pub speciation_add_best_member_back_in: bool,
    pub speciation_preserve_elite: bool,
    pub speciation_min_threshold: NeatFloat,
    pub speciation_max_threshold: NeatFloat,
    pub speciation_species_min_number_of_members: usize,
    pub speciation_cross_species_reproduction_scale: NeatFloat,
    pub hyperneat_substrate_set: Option<SubstrateSet>,
    pub reproduction_weights_from_fitter_probability: NeatFloat,
    pub print_summary_interval: Option<u32>,
    pub print_summary_number_of_species_to_show: usize,
    pub success_threshold: NeatFloat,
    pub event_subscriptions: Vec<EventSubscription>,
    pub run_save_directory: String,
    pub run_name: String
}

impl Configuration{
    pub fn neat(node_genes: Box<Vec<NodeGene>>, success_threshold: NeatFloat) -> Self {
        if !node_genes.len() == 0{
            panic!("Must have node genes");
        }

        if !node_genes.iter().map(|n| n.number).collect::<Vec<i32>>().is_unique(){
            panic!("Node genes must have unique indexes");
        }

        if !node_genes.iter().any(|n| n.kind == NodeKind::Sensor){
            panic!("Configuration does not contain any sensors");
        }

        let first_sensor = node_genes.iter().find(|n| n.kind == NodeKind::Sensor);
        if first_sensor.unwrap().number != 1{
            panic!("Sensors must start at index 1");
        }

        let mut expected_index = 1;
        for n in node_genes.iter() {
            if n.number != expected_index {
                panic!("node indexes should be contiguous. for example [1, 2, 3]");
            }
            expected_index +=1;
        }

        if !node_genes.iter().any(|n| n.kind == NodeKind::Output){
            panic!("Configuration does not contain any outputs");
        }

        let initial_count = node_genes.iter().map(|n| n.number).max().unwrap() as usize;
        let mut configuration = ConfigurationDefaults::get();
        configuration.node_genes = node_genes;
        configuration.success_threshold = success_threshold;
        configuration
    }
    pub fn hyperneat(substrate_set: SubstrateSet, success_threshold: NeatFloat) -> Self {
        let mut configuration = ConfigurationDefaults::get()
        .mutation_node_available_activation_functions_for_cppn();
        let node_conf = substrate_set.get_cpp_node_conf();
        configuration.hyperneat_substrate_set = Some(substrate_set);
        configuration.success_threshold = success_threshold;
        configuration.node_genes = node_conf;
        configuration
    }
    
    pub fn set_mutation_config(original: &Configuration, outputs: &Vec<NeatFloat>) -> Self{
        let mut new_config = original.clone();
        new_config.mutation_node_add_probability = outputs[0];
        new_config.mutation_node_delete_probability = outputs[1];
        new_config.mutation_connection_add_probability = outputs[2];
        new_config.mutation_connection_delete_probability = outputs[3];
        new_config.mutation_connection_weight_change_probability = outputs[4];
        new_config.mutation_connection_weight_change_scale = outputs[5];
        new_config.mutation_connection_weight_replace_probability = outputs[6];
        new_config.mutation_node_bias_change_probability = outputs[7];
        new_config.mutation_node_bias_change_scale = outputs[8];
        new_config
    }
    pub fn mutation_no_mutation(mut self) -> Self {
        self.mutation_node_add_probability = 0.0;
        self.mutation_node_delete_probability = 0.0;
        self.mutation_connection_add_probability = 0.0;
        self.mutation_connection_delete_probability = 0.0;
        self.mutation_connection_weight_change_probability = 0.0;
        self.mutation_connection_weight_replace_probability = 0.0;
        self.mutation_node_bias_change_probability = 0.0;
        self
    }
    pub fn genome_minimal_genes_to_connect_ratio(mut self, value: NeatFloat) -> Self {
        self.genome_minimal_genes_to_connect_ratio = value;
        self
    }
    pub fn mutation_node_add_probability(mut self, value: NeatFloat) -> Self {
        self.mutation_node_add_probability = value;
        self
    }
    pub fn mutation_node_add_mode(mut self, value: MutationNodeAddMode) -> Self {
        self.mutation_node_add_mode = value;
        self
    }
    pub fn mutation_node_delete_probability(mut self, value: NeatFloat) -> Self {
        self.mutation_node_delete_probability = value;
        self
    }
    pub fn mutation_connection_add_probability(mut self, value: NeatFloat) -> Self {
        self.mutation_connection_add_probability = value;
        self
    }
    pub fn mutation_connection_delete_probability(mut self, value: NeatFloat) -> Self {
        self.mutation_connection_delete_probability = value;
        self
    }
    pub fn mutation_connection_weight_change_probability(mut self, value: NeatFloat) -> Self {
        self.mutation_connection_weight_change_probability = value;
        self
    }
    pub fn mutation_connection_allow_recurrent(mut self, value: bool) -> Self {
        self.mutation_connection_allow_recurrent = value;
        self
    }
    pub fn mutation_connection_weight_change_scale(mut self, value: NeatFloat) -> Self {
        self.mutation_connection_weight_change_scale = value;
        self
    }
    pub fn mutation_connection_weight_replace_probability(mut self, value: NeatFloat) -> Self {
        self.mutation_connection_weight_replace_probability = value;
        self
    }
    pub fn mutation_node_bias_change_probability(mut self, value: NeatFloat) -> Self {
        self.mutation_node_bias_change_probability = value;
        self
    }
    pub fn mutation_node_bias_change_scale(mut self, value: NeatFloat) -> Self {
        self.mutation_node_bias_change_scale = value;
        self
    }
    pub fn mutation_node_cppn_input_multiplier_change_scale(mut self, value: NeatFloat) -> Self {
        self.mutation_node_cppn_input_multiplier_change_scale = value;
        self
    }
    pub fn mutation_node_cppn_input_multiplier_change_probability(mut self, value: NeatFloat) -> Self {
        self.mutation_node_cppn_input_multiplier_change_probability = value;
        self
    }
    pub fn mutation_node_cppn_input_multiplier_replace_probability(mut self, value: NeatFloat) -> Self {
        self.mutation_node_cppn_input_multiplier_replace_probability = value;
        self
    }
    pub fn mutation_remove_unconnected_nodes(mut self, value: bool) -> Self {
        self.mutation_remove_unconnected_nodes = value;
        self
    }
    pub fn mutation_connection_disable_probability(mut self, value: NeatFloat) -> Self {
        self.mutation_connection_disable_probability = value;
        self
    }
    pub fn mutation_node_available_activation_functions(mut self, value: ActivationFunction) -> Self {
        self.mutation_node_available_activation_functions = value;
        self
    }
    pub fn mutation_node_available_activation_functions_for_cppn(mut self) -> Self {
        self.mutation_node_available_activation_functions = ActivationFunction::for_cppn();
        self
    }
    pub fn speciation_genetic_distance_threshold(mut self, value: NeatFloat) -> Self {
        self.speciation_genetic_distance_threshold = value;
        self
    }
    pub fn speciation_drop_species_no_improvement_generations(mut self, value: u32) -> Self {
        self.speciation_drop_species_no_improvement_generations = value;
        self
    }
    pub fn speciation_remove_stagnant_species_generations(mut self, value: u32) -> Self {
        self.speciation_remove_stagnant_species_generations = value;
        self
    }
    pub fn speciation_add_new_species_during_run(mut self, value: bool) -> Self {
        self.speciation_add_new_species_during_run = value;
        self
    }
    pub fn speciation_new_species_protected_for_generations(mut self, value: u32) -> Self {
        self.speciation_new_species_protected_for_generations = value;
        self
    }
    pub fn speciation_use_best_seed_bank(mut self, value: Option<usize>) -> Self {
        self.speciation_use_best_seed_bank = value;
        self
    }
    pub fn speciation_add_best_member_back_in(mut self, value: bool) -> Self {
        self.speciation_add_best_member_back_in = value;
        self
    }
    pub fn speciation_offspring_mode(mut self, value: OffSpringMode) -> Self {
        self.speciation_offspring_mode = value;
        self
    }
    pub fn speciation_offspring_outcome_novelty_weight(mut self, value: NeatFloat) -> Self {
        self.speciation_offspring_outcome_novelty_weight = value;
        self
    }
    pub fn speciation_preserve_elite(mut self, value: bool) -> Self {
        self.speciation_preserve_elite = value;
        self
    }
    pub fn speciation_max_threshold(mut self, value: NeatFloat) -> Self {
        self.speciation_max_threshold = value;
        self
    }
    pub fn speciation_min_threshold(mut self, value: NeatFloat) -> Self {
        self.speciation_min_threshold = value;
        self
    }
    pub fn speciation_species_min_number_of_members(mut self, value: usize) -> Self {
        self.speciation_species_min_number_of_members = value;
        self
    }
    /**
        speciation_cross_species_reproduction_scale should be kept quite small like 0.01
    */
    pub fn speciation_cross_species_reproduction_scale(mut self, value: NeatFloat) -> Self {
        self.speciation_cross_species_reproduction_scale = value;
        self
    }
    pub fn reproduction_weights_from_fitter_probability(mut self, value: NeatFloat) -> Self {
        self.reproduction_weights_from_fitter_probability = value;
        self
    }
    pub fn population_size(mut self, value: u32) -> Self {
        self.population_size = value;
        self
    }
    pub fn target_species(mut self, value: u32) -> Self {
        self.target_species = value;
        self
    }
    pub fn survival_threshold(mut self, value: NeatFloat) -> Self {
        self.survival_threshold = value;
        self
    }
    pub fn success_threshold(mut self, value: NeatFloat) -> Self {
        self.success_threshold = value;
        self
    }
    pub fn print_summary_interval(mut self, value: Option<u32>) -> Self {
        self.print_summary_interval = value;
        self
    }
    pub fn print_summary_number_of_species_to_show(mut self, value: usize) -> Self {
        self.print_summary_number_of_species_to_show = value;
        self
    }
    pub fn add_event_subscription(mut self, event_type: EventType, listeners: Listeners) -> Self{
        self.event_subscriptions.push(EventSubscription{
            event_type, 
            listeners
        });
        self
    }
    pub fn run_save_directory(mut self, value: String) -> Self{
        self.run_save_directory = value;
        self
    }
    pub fn run_name(mut self, value: String) -> Self{
        self.run_name = value;
        self
    }
    

}


#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, PartialEq)]
pub enum OffSpringMode {
    Average,
    AdjustedMemberRange,
    AdjustedSpeciesRange
}

#[test]
fn nodes_must_have_unique_indexes() {
    let gene_table = ConnectGeneTable::new();

    let test =|| {
        let configuration  = Configuration::neat(
            Box::new(vec![
                NodeGene::new(1, NodeKind::Sensor),
                NodeGene::new(1, NodeKind::Output)
            ]), 0.0);
    };
   
    let result = std::panic::catch_unwind(test);
    assert!(result.is_err());  //probe further for specific error type here, if desired
}

#[test]
fn nodes_must_have_nodes() {
    let gene_table = ConnectGeneTable::new();

    let test =|| {
        let configuration  = Configuration::neat(
            Box::new(vec![
            ]), 0.0);
    };
   
    let result = std::panic::catch_unwind(test);
    assert!(result.is_err());  //probe further for specific error type here, if desired
}

#[test]
fn property_setters_work() {
    let gene_table = ConnectGeneTable::new();

    let configuration  = Configuration::neat(
        Box::new(vec![
            NodeGene::new(1, NodeKind::Sensor),
            NodeGene::new(2, NodeKind::Output)
        ]), 0.0)
        .mutation_node_add_probability(0.1)
        .mutation_node_delete_probability(0.2)
        .mutation_connection_add_probability(0.3)
        .mutation_connection_delete_probability(0.4);
   
    assert_eq!(configuration.mutation_node_add_probability, 0.1);
    assert_eq!(configuration.mutation_node_delete_probability, 0.2);
    assert_eq!(configuration.mutation_connection_add_probability, 0.3);
    assert_eq!(configuration.mutation_connection_delete_probability, 0.4);
}