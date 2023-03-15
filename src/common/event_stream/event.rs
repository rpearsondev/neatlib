use serde::{Serialize, Deserialize};
use bitflags::bitflags;
pub type PropertyType = gluesql::core::data::Value;

use crate::{neat::{trainer::run_context::RunContext, genome::neat::NeatGenome, population::speciation::{species_member::SpeciesMember}}, activation_functions::ActivationFunction, common::NeatFloat};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Event{
    pub event_type: EventType,
    pub generation: u32,
    pub additional_properties: Vec<(String, PropertyType)>
}

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct EventType: u32{
        const MUTATION_NODE_ADD = 1;
        const MUTATION_NODE_DELETE = 2; 
        const MUTATION_CONNECTION_ADD = 4;   
        const MUTATION_CONNECTION_DELETE = 8;
        const MUTATION_NODE_CHANGE_BIAS = 16;   
        const MUTATION_CONNECTION_WEIGHT_CHANGE = 32; 
        const MUTATION_CONNECTION_WEIGHT_REPLACE = 64;
        const MUTATION_CONNECTION_DISABLED = 128;
        const SPECIATION_REPRODUCE = 256;
        const SPECIATION_REPRODUCE_CROSS_SPECIES = 512;
        const SPECIATION_SPECIES_REMOVE = 1024;
        const SPECIATION_SPECIES_NEW = 2048;
        const SPECIATION_SURVIVOR = 4096;
    }
}
impl EventType {
    pub fn get_all() -> [EventType; 13]{
        [
            EventType::MUTATION_NODE_ADD, 
            EventType::MUTATION_NODE_DELETE, 
            EventType::MUTATION_CONNECTION_ADD, 
            EventType::MUTATION_CONNECTION_DELETE, 
            EventType::MUTATION_NODE_CHANGE_BIAS, 
            EventType::MUTATION_CONNECTION_WEIGHT_CHANGE,
            EventType::MUTATION_CONNECTION_WEIGHT_REPLACE,
            EventType::MUTATION_CONNECTION_DISABLED,
            EventType::SPECIATION_REPRODUCE,
            EventType::SPECIATION_REPRODUCE_CROSS_SPECIES,
            EventType::SPECIATION_SPECIES_REMOVE,
            EventType::SPECIATION_SPECIES_NEW,
            EventType::SPECIATION_SURVIVOR,
        ]
      }
}

impl Event{
    pub fn mutation_node_add(run_context: &RunContext, genome_id: &uuid::Uuid, node_number: i32, activation_function: ActivationFunction ,connection_in: i32, connection_out: i32) -> Self{
        
        Event{ 
            event_type: EventType::MUTATION_NODE_ADD, 
            generation: run_context.current_generation, 
            additional_properties: vec![
                ("genome_id".to_string(), PropertyType::Uuid(genome_id.as_u128()) ),
                ("node".to_string(), PropertyType::I32(node_number) ),
                ("activation".to_string(), PropertyType::Str(format!("{:?}", activation_function)) ),
                ("connect_in".to_string(), PropertyType::I32(connection_in) ),
                ("connect_out".to_string(), PropertyType::I32(connection_out) )
            ] 
        }
    }
    pub fn mutation_node_delete(run_context: &RunContext, genome_id: &uuid::Uuid, node_number: i32) -> Self{
        Event{ 
            event_type: EventType::MUTATION_NODE_DELETE, 
            generation: run_context.current_generation, 
            additional_properties: vec![
                ("genome_id".to_string(), PropertyType::Uuid(genome_id.as_u128())),
                ("node".to_string(), PropertyType::I32(node_number)  )
            ] }
    }
    pub fn mutation_connection_add(run_context: &RunContext, genome_id: &uuid::Uuid, connection_in: i32, connection_out: i32) -> Self{
        Event{ 
            event_type: EventType::MUTATION_CONNECTION_ADD, 
            generation: run_context.current_generation, 
            additional_properties: vec![
                ("genome_id".to_string(), PropertyType::Uuid(genome_id.as_u128()) ),
                ("connect_in".to_string(), PropertyType::I32(connection_in) ),
                ("connect_out".to_string(), PropertyType::I32(connection_out) )
            ] }
    }
    pub fn mutation_connection_delete(run_context: &RunContext, genome_id: &uuid::Uuid, connection_in: i32, connection_out: i32) -> Self{
        Event{ 
            event_type: EventType::MUTATION_CONNECTION_DELETE, 
            generation: run_context.current_generation, 
            additional_properties: vec![
                ("genome_id".to_string(), PropertyType::Uuid(genome_id.as_u128()) ),
                ("connect_in".to_string(), PropertyType::I32(connection_in) ),
                ("connect_out".to_string(), PropertyType::I32(connection_out) )
            ] }
    }
    pub fn mutation_node_bias_change(run_context: &RunContext, genome_id: &uuid::Uuid, node_number: i32, new_bias: NeatFloat) -> Self{
        Event{ 
            event_type: EventType::MUTATION_NODE_CHANGE_BIAS, 
            generation: run_context.current_generation, 
            additional_properties: vec![
                ("genome_id".to_string(), PropertyType::Uuid(genome_id.as_u128()) ),
                ("node".to_string(), PropertyType::I32(node_number) ),
                ("new_bias".to_string(), PropertyType::F64(new_bias as f64) )
            ] }
    }
    pub fn mutation_connection_weight_change(run_context: &RunContext, genome_id: &uuid::Uuid, connection_in: i32, connection_out: i32, new_weight: NeatFloat, old_weight: NeatFloat) -> Self{
        Event{ 
            event_type: EventType::MUTATION_CONNECTION_WEIGHT_CHANGE, 
            generation: run_context.current_generation, 
            additional_properties: vec![
                ("genome_id".to_string(), PropertyType::Uuid(genome_id.as_u128()) ),
                ("connect_in".to_string(), PropertyType::I32(connection_in) ),
                ("connect_out".to_string(), PropertyType::I32(connection_out) ),
                ("new_weight".to_string(), PropertyType::F64(new_weight as f64) ),
                ("old_weight".to_string(), PropertyType::F64(old_weight as f64) )
            ] }
    }
    pub fn mutation_connection_weight_replace(run_context: &RunContext, genome_id: &uuid::Uuid, connection_in: i32, connection_out: i32, new_weight: NeatFloat, old_weight: NeatFloat) -> Self{
        Event{ 
            event_type: EventType::MUTATION_CONNECTION_WEIGHT_REPLACE, 
            generation: run_context.current_generation, 
            additional_properties: vec![
                ("genome_id".to_string(), PropertyType::Uuid(genome_id.as_u128()) ),
                ("connect_in".to_string(), PropertyType::I32(connection_in) ),
                ("connect_out".to_string(), PropertyType::I32(connection_out) ),
                ("new_weight".to_string(), PropertyType::F64(new_weight as f64) ),
                ("old_weight".to_string(), PropertyType::F64(old_weight as f64) )
            ] }
    }
    pub fn mutation_connection_disabled(run_context: &RunContext, genome_id: &uuid::Uuid, connection_in: i32, connection_out: i32) -> Self{
        Event{ 
            event_type: EventType::MUTATION_CONNECTION_DISABLED, 
            generation: run_context.current_generation, 
            additional_properties: vec![
                ("genome_id".to_string(), PropertyType::Uuid(genome_id.as_u128()) ),
                ("connect_in".to_string(), PropertyType::I32(connection_in) ),
                ("connect_out".to_string(), PropertyType::I32(connection_out) ),
            ] }
    }
    pub fn speciation_reproduce(current_generation: u32, best_genome: &NeatGenome, other_genome: &NeatGenome, new_genome: &NeatGenome, species_id: &uuid::Uuid) -> Self{
        Event{ 
            event_type: EventType::SPECIATION_REPRODUCE, 
            generation: current_generation, 
            additional_properties: vec![
                ("best_genome_id".to_string(), PropertyType::Uuid(best_genome.id.as_u128()) ),
                ("other_genome_id".to_string(), PropertyType::Uuid(other_genome.id.as_u128()) ),
                ("new_genome".to_string(), PropertyType::Uuid(new_genome.id.as_u128()) ),
                ("species_id".to_string(), PropertyType::Uuid(species_id.as_u128()) ),
            ] }
    }  
    pub fn speciation_reproduce_cross_species(current_generation: u32, best_genome: &NeatGenome, other_genome: &NeatGenome, new_genome: &NeatGenome, species_id: &uuid::Uuid) -> Self{
        Event{ 
            event_type: EventType::SPECIATION_REPRODUCE_CROSS_SPECIES, 
            generation: current_generation, 
            additional_properties: vec![
                ("best_genome_id".to_string(), PropertyType::Uuid(best_genome.id.as_u128()) ),
                ("best_genome_complexity".to_string(), PropertyType::F64(best_genome.genes.get_complexity() as f64) ),
                ("other_genome_id".to_string(), PropertyType::Uuid(other_genome.id.as_u128()) ),
                ("other_genome_complexity".to_string(), PropertyType::F64(other_genome.genes.get_complexity() as f64) ),
                ("new_genome".to_string(), PropertyType::Uuid(new_genome.id.as_u128()) ),
                ("new_genome_complexity".to_string(), PropertyType::F64(new_genome.genes.get_complexity() as f64) ),
                ("species_id".to_string(), PropertyType::Uuid(species_id.as_u128()) ),
            ] }
    } 

    
    pub fn species_species_remove_no_offspring(current_generation: u32, species_id: &uuid::Uuid) -> Self{
        Event{ 
            event_type: EventType::SPECIATION_SPECIES_REMOVE, 
            generation: current_generation, 
            additional_properties: vec![
                ("species_id".to_string(), PropertyType::Uuid(species_id.as_u128()) ),
                ("reason".to_string(), PropertyType::Str("no_offspring".to_string()) ),
            ] }
    }
    pub fn species_species_remove_no_improvement(current_generation: u32, species_id: &uuid::Uuid) -> Self{
        Event{ 
            event_type: EventType::SPECIATION_SPECIES_REMOVE, 
            generation: current_generation, 
            additional_properties: vec![
                ("species_id".to_string(), PropertyType::Uuid(species_id.as_u128()) ),
                ("reason".to_string(), PropertyType::Str("no_improvement".to_string()) ),
            ] }
    }
    pub fn species_species_new(current_generation: u32, species_id: &uuid::Uuid) -> Self{
        Event{ 
            event_type: EventType::SPECIATION_SPECIES_NEW, 
            generation: current_generation, 
            additional_properties: vec![
                ("species_id".to_string(), PropertyType::Uuid(species_id.as_u128()) ),
            ] }
    }
    pub fn species_survivor(current_generation: u32, species_id: &uuid::Uuid, member: &SpeciesMember) -> Self{
        Event{ 
            event_type: EventType::SPECIATION_SURVIVOR, 
            generation: current_generation, 
            additional_properties: vec![
                ("species_id".to_string(), PropertyType::Uuid(species_id.as_u128()) ),
                ("genome_id".to_string(), PropertyType::Uuid(member.id.as_u128()) ),
                ("objective_fitness".to_string(), PropertyType::F64(member.objective_fitness as f64) ),
            ] }
    }       
}