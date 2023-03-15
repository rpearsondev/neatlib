use bevy::prelude::Color;
use hashbrown::HashMap;
use uuid::Uuid;
use crate::common::NeatFloat;
use crate::common::network_definition::{NetworkDefinition, NetworkDefinitionNode, NetworkDefinitionConnection};
use crate::common::random::Random;
use crate::neat::trainer::configuration::Configuration;
use crate::neat::trainer::run_context::RunContext;
use crate::node_kind::NodeKind;
use crate::neat::genome::neat::mutation_mode::MutationMode;
use super::connect_gene::ConnectGene;
use super::connect_genes::ConnectGenes;
use super::super::genome::Genome;
use super::mutation::Mutation;
use super::node_gene::{NodeGene};
use super::node_genes::NodeGenes;
use super::neat_genes::NeatGenes;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Default)]
#[derive(Serialize, Deserialize)]
pub struct NeatGenome {
    pub id: Uuid,
    pub genes: NeatGenes,
    pub parents_objective_fitness: Option<NeatFloat>,
    pub objective_fitness: Option<NeatFloat>,
    pub novelty: NeatFloat,
    pub allow_mutation: bool,
    pub cached_genetic_distance: HashMap<uuid::Uuid, NeatFloat>,
    pub color: Color,
    pub mutations: Vec<Mutation>
}

impl NeatGenome{
    pub fn mutate_color(&mut self){
        let change: NeatFloat = Random::standard_normal() * 0.01;
        let new_val = (self.color.r() + change).clamp(0.0, 1.0);
        self.color.set_r(new_val);
        
        let change: NeatFloat = Random::standard_normal() * 0.01;
        let new_val = (self.color.g() + change).clamp(0.0, 1.0);
        self.color.set_g(new_val);

        let change: NeatFloat = Random::standard_normal() * 0.01;
        let new_val = (self.color.b() + change).clamp(0.0, 1.0);
        self.color.set_b(new_val);
    }
}

impl Genome for NeatGenome{
    fn minimal(configuration: &Configuration, run_context: &mut RunContext) -> Self{
        
        let outputs = configuration.node_genes.iter().filter(|n| n.kind == NodeKind::Output).collect::<Vec<&NodeGene>>();
        let sensors = configuration.node_genes.iter().filter(|n| n.kind == NodeKind::Sensor).collect::<Vec<&NodeGene>>();
        let mut connect_genes:  Vec<ConnectGene> = Vec::new();

        for output in outputs{
            for sensor in &sensors{
                let random = Random::gen_range_f32(0.0 ,1.0);
                if random <= configuration.genome_minimal_genes_to_connect_ratio{
                    let gene_to_add = ConnectGene::new(sensor.number, output.number, true);
                    let connection= run_context.gene_table.add(gene_to_add);
                    connect_genes.push(connection);
                }
            }
        }
        NeatGenome{
            id: Uuid::new_v4(),
            genes: NeatGenes::new(NodeGenes::from_vec(&*configuration.node_genes), ConnectGenes::from_vec(&connect_genes)),
            parents_objective_fitness: None,            
            objective_fitness: None,
            novelty: 0.0,
            allow_mutation: true,
            cached_genetic_distance: HashMap::new(),
            color: Color::Rgba { red: Random::gen_range_f32(0.0, 1.0), green: Random::gen_range_f32(0.0, 1.0), blue: Random::gen_range_f32(0.0, 1.0), alpha: 1.0 },
            mutations: Vec::new()
        }
    }
    fn mutate(&mut self, configuration: &Configuration, run_context: &mut RunContext, mutation_mode: MutationMode) {
        if self.allow_mutation{
            self.genes.mutate(&self.id, configuration, run_context, &mut self.mutations, mutation_mode);
        }
        NeatGenome::mutate_color(self);
    }
    fn dont_allow_mutation(&mut self){
        self.allow_mutation = false;
    }
    fn set_objective_fitness(&mut self, value: NeatFloat){
        self.objective_fitness = Some(value);
    }
    fn set_novelty(&mut self, value: NeatFloat){
        self.novelty = value;
    }
    fn get_genetic_difference_distance_from(&mut self, other_genome: &NeatGenome, stop_when_hit: NeatFloat) -> NeatFloat{
        let cached_result = self.cached_genetic_distance.get(&other_genome.id);
        if cached_result.is_some(){
            return *cached_result.unwrap();
        }

        let result = self.genes.get_genetic_difference_distance_from(&other_genome.genes, stop_when_hit);
        self.cached_genetic_distance.insert(other_genome.id, result);
        result
    }
    fn get_fitness(&self) -> Option<NeatFloat> {
        self.objective_fitness 
    }

    fn get_id(&self) -> uuid::Uuid {
        self.id
    }

    fn get_complexity(&self) -> NeatFloat {
        self.genes.get_complexity()
    }
}

impl NetworkDefinition for NeatGenome{
    fn get_network_identifier(&self) -> Uuid {
        
        self.get_id()
    }

    fn get_node(&self, node_identity: i32) -> NetworkDefinitionNode {
        
        let (node, pos) = self.genes.nodes.get_with_position(node_identity);
        NetworkDefinitionNode{
            node_position: pos,
            activation_function: node.activation_function,
            bias: node.bias,
            input_multiplier: node.input_multiplier,
            identity: node.number,
            kind: node.kind.clone(),
            position: None
        }
    }

    fn get_all_nodes(&self) -> Vec<NetworkDefinitionNode> {
        
        self.genes.nodes.iter().enumerate().map(|(i, node)|{
            NetworkDefinitionNode{
                node_position: i,
                activation_function: node.activation_function,
                bias: node.bias,
                input_multiplier: node.input_multiplier,
                identity: node.number,
                kind: node.kind.clone(),
                position: None
            } 
        }
        ).collect::<Vec<NetworkDefinitionNode>>()
    }

    fn get_all_connections(&self) -> Vec<NetworkDefinitionConnection> {
        
        self.genes.connect.iter().filter(|c| c.is_enabled ).map(|connection| 
            NetworkDefinitionConnection {
                connection_in: connection.connection_in,
                connection_out: connection.connection_out,
                is_enabled: connection.is_enabled,
                is_recurrent: connection.is_recurrent,
                weight: connection.weight,

            }
        ).collect::<Vec<NetworkDefinitionConnection>>()
    }

    fn get_feed_connections_for_node(&self, node_identity: i32) -> Vec<NetworkDefinitionConnection> {
        
        self.genes.connect.iter()
        .filter(|c| c.connection_out == node_identity && c.is_enabled)
        .map(|connection| 
            NetworkDefinitionConnection {
                connection_in: connection.connection_in,
                connection_out: connection.connection_out,
                is_enabled: connection.is_enabled,
                is_recurrent: connection.is_recurrent,
                weight: connection.weight,

            }
        ).collect::<Vec<NetworkDefinitionConnection>>()
    }

    fn get_nodes_len(&self) -> u32 {
        
        self.genes.nodes.len() as u32
    }

    fn get_output_nodes_count(&self) -> u32 {
        
        self.genes.nodes.iter().filter(|n| n.kind == NodeKind::Output).count() as u32
    }
}