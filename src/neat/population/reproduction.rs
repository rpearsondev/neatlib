use hashbrown::HashMap;
use rand::Rng;
use rayon::iter::IntoParallelRefIterator;
use super::super::{genome::neat::{NeatGenome, connect_gene::ConnectGene, node_gene::NodeGene, node_genes::NodeGenes, connect_genes::ConnectGenes, neat_genes::NeatGenes}};
use super::GenerationMember;
use crate::{phenome::Phenome, common::{random::Random, NeatFloat}};
pub struct Reproduction {
}

impl Reproduction {
    pub fn reproduce(best_performing: &NeatGenome, other: &NeatGenome, species_hint: uuid::Uuid, reproduction_weights_from_fitter_probability: NeatFloat) -> NeatGenome {
        let mut rng = rand::thread_rng();
        
        let best_connect_genes = &best_performing.genes.connect.to_vec();
        let best_connect_len = best_connect_genes.len(); 
        let other_connect_genes = &other.genes.connect;
        
        let mut new_connect_genes: Vec<ConnectGene> = Vec::with_capacity(best_connect_len);
        for i in 0..best_connect_len {
            let best_gene = &best_connect_genes[i];
            let other_gene = other_connect_genes.get_by_hash(best_gene.connection_hash);
            if other_gene.is_some() {
                if Random::gen_bool(reproduction_weights_from_fitter_probability)
                {
                    new_connect_genes.push(best_gene.clone())
                } else {
                    new_connect_genes.push((*(other_gene.unwrap())).clone())
                }
            }
        }

        let best_node_genes = &best_performing.genes.nodes;
        let best_node_len = best_node_genes.len(); 
        let other_node_genes = &other.genes.nodes;
        let mut new_node_genes: Vec<NodeGene> = Vec::with_capacity(best_node_genes.len());
        for best_gene in best_node_genes.iter() {
            let other_gene = other_node_genes.get_opt(best_gene.number);
            if other_gene.is_some() {
                if Random::gen_bool(reproduction_weights_from_fitter_probability)
                {
                    new_node_genes.push(best_gene.clone())
                } else {
                    new_node_genes.push((*(other_gene.unwrap())).clone())
                }
            }
        }

        let new_genome = NeatGenome{
            objective_fitness: None,
            parents_objective_fitness: best_performing.objective_fitness,
            novelty: 0.0,
            id: uuid::Uuid::new_v4(),
            genes: NeatGenes::new(NodeGenes::from_vec(&new_node_genes), ConnectGenes::from_vec(&new_connect_genes)),
            allow_mutation: true,
            cached_genetic_distance: HashMap::new(),
            color: best_performing.color,
            mutations: Vec::new()
        };
        new_genome
    }
    pub fn reproduce_cross_species(best_performing: &NeatGenome, other: &NeatGenome, species_hint: uuid::Uuid, reproduction_weights_from_fitter_probability: NeatFloat) -> NeatGenome {
        let best_connect_genes = best_performing.genes.connect.to_vec();
        let other_connect_genes = other.genes.connect.to_vec();
        let max_limit_connection = (best_connect_genes.len() + other_connect_genes.len()) / 2;
        //let max_limit_connection = usize::min(best_connect_genes.len(), other_connect_genes.len());
        
        let mut new_connect_genes: Vec<ConnectGene> = Vec::with_capacity(best_connect_genes.len() + other_connect_genes.len());
        for gene in best_connect_genes.iter() {
            new_connect_genes.push(gene.clone());
        }
        for gene in other_connect_genes.iter() {
            if new_connect_genes.iter().find(|g|g.connection_hash == gene.connection_hash).is_none(){
                new_connect_genes.push(gene.clone());    
            }
        }

        new_connect_genes.sort_by(|a, b| a.weight.abs().partial_cmp(&b.weight.abs()).unwrap());
        while(new_connect_genes.len() > (max_limit_connection as NeatFloat * 0.99) as usize){
            new_connect_genes.remove(0);
        }

     
        let best_node_genes = best_performing.genes.nodes.iter();
        let other_node_genes = other.genes.nodes.iter();
        let mut new_node_genes: Vec<NodeGene> = Vec::with_capacity(best_performing.genes.nodes.len() + other.genes.nodes.len());
        for gene in best_node_genes {
            new_node_genes.push(gene.clone());
        }
        for gene in other_node_genes {
            if new_node_genes.iter().find(|g|g.number == gene.number).is_none(){
                new_node_genes.push(gene.clone());
            }
        }

        let mut new_genome = NeatGenome{
            objective_fitness: None,
            parents_objective_fitness: best_performing.objective_fitness,
            novelty: 0.0,
            id: uuid::Uuid::new_v4(),
            genes: NeatGenes::new(NodeGenes::from_vec(&new_node_genes), ConnectGenes::from_vec(&new_connect_genes)),
            allow_mutation: true,
            cached_genetic_distance: HashMap::new(),
            color: best_performing.color,
            mutations: Vec::new()
        };

        new_genome.genes.cleanup_orphan_nodes();
        new_genome
    }
}