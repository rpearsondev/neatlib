#![allow(unused)]

use std::cmp::Ordering;
use serde::{Serialize, Deserialize};
use serde::{};
use super::genome::genome::Genome;
use crate::common::NeatFloat;
use crate::phenome::Phenome;
use super::{ 
    genome::neat::{NeatGenome, node_gene::NodeGene}, 
    trainer::{configuration::Configuration, node_conf::NodeConf}
};
pub mod reproduction;
pub mod speciation;
pub mod members_lookup;

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct GenerationMember <T> where T: Genome{
    pub genome: T,
    pub created_generation: u32,
    pub number_of_generations_since_species_improved: u32,
    #[serde(skip)]
    pub species_hint: uuid::Uuid
}

impl<T>  GenerationMember<T> where T: Genome{
    pub fn new(genome: T, created_generation: u32) -> Self{
        GenerationMember{
            genome,
            created_generation,
            number_of_generations_since_species_improved: 0,
            species_hint: uuid::Uuid::nil()
        }
    }
}

impl<T> Ord for GenerationMember<T> where T: Genome{
    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.genome.get_fitness();
        let b = other.genome.get_fitness();

        let x  = b.partial_cmp(&a).unwrap();
        x
    }
}
impl<T> PartialOrd for GenerationMember<T> where T: Genome {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.genome.get_fitness().partial_cmp(&self.genome.get_fitness())
    }
}
impl<T> PartialEq for GenerationMember<T> where T: Genome {
    fn eq(&self, other: &Self) -> bool {
        (self.genome.get_id()) == (other.genome.get_id())
    }
}
impl<T> Eq for GenerationMember<T> where T: Genome { }
