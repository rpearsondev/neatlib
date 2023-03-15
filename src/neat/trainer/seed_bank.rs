use serde::{Deserialize, Serialize};
use crate::{neat::population::GenerationMember, neat::genome::genome::Genome, common::NeatFloat};

#[derive(Serialize, Deserialize, Clone)]
pub struct SeedBank<T> where T: Genome, T: Clone{
    pub seeds: Vec<GenerationMember<T>>,
    pub limit: usize,
    pub lowest_fitness: Option<NeatFloat>
}

impl<T> SeedBank<T> where T: Genome ,T: Clone{
    pub fn new(limit: usize) -> Self{
        Self {
            seeds: vec![],
            limit: limit,
            lowest_fitness: None
        }
    }
    pub fn update_seed_bank(&mut self, members: &Vec<GenerationMember<T>>){
        if self.limit <= 0{
            return;
        }
        for member in members{
            let fitness = member.genome.get_fitness();

            if fitness.is_none(){
                continue;
            }

            let fitness: NeatFloat = fitness.unwrap();
            if self.lowest_fitness.is_none() || self.lowest_fitness.unwrap() < fitness{
                self.seeds.push((*member).clone());
                self.lowest_fitness = Some(fitness);
            }
        }
        if self.seeds.len() > self.limit{
            self.seeds.sort();
            for i in (self.limit-1..self.seeds.len()-1).rev(){
                self.seeds.remove(i);
            }
           
            let last = self.seeds.iter().last();
            if last.is_some(){
                self.lowest_fitness = last.unwrap().genome.get_fitness()
            }
        }
    }
}