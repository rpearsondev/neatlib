use std::slice::{Iter, IterMut};
use hashbrown::HashMap;
use nohash_hasher::BuildNoHashHasher;
use serde::{Serialize, Deserialize};
use crate::common::random::Random;

use super::connect_gene::ConnectGene;

#[derive(Serialize, Deserialize)]
#[derive(Clone, Default, Debug)]
pub struct ConnectGenes {
    connect_genes: Vec<ConnectGene>,
    map: HashMap<u64, usize, nohash_hasher::BuildNoHashHasher<u64>>
}

impl ConnectGenes{
    pub fn add(&mut self, connect_gene:ConnectGene){
        
        self.map.insert(connect_gene.connection_hash, self.connect_genes.len());
        self.connect_genes.push(connect_gene);
    }
    pub fn delete(&mut self, connection_in: i32, connection_out:i32){
        
        let position = self.map.get(&ConnectGene::compute_hash(connection_in, connection_out));
        if position.is_some(){
            let pos = *position.unwrap();
            self.connect_genes.remove(pos);
            self.recompute_indexes();
        }
    }
    pub fn delete_connections_that_connect_to_node(&mut self, in_or_out: i32){
        
        self.connect_genes.retain(|c| c.connection_in != in_or_out && c.connection_out != in_or_out);
        self.recompute_indexes();
    }
    pub fn get_unchecked(&self, connection_in: i32, connection_out:i32 ) -> &ConnectGene{
        
        let connect_hash = ConnectGene::compute_hash(connection_in, connection_out);
        let position = self.map.get(&connect_hash);
        return &self.connect_genes.get(*position.unwrap()).unwrap();
    }
    pub fn get(&self, connection_in: i32, connection_out:i32 ) -> Option<&ConnectGene>{
        
        let connect_hash = ConnectGene::compute_hash(connection_in, connection_out);
        let position = self.map.get(&connect_hash);
        if position.is_none(){
            return None;
        }
        return self.connect_genes.get(*position.unwrap());
    }
    pub fn get_mut(&mut self, connection_in: i32, connection_out:i32 ) -> Option<&mut ConnectGene>{
        
        let connect_hash = ConnectGene::compute_hash(connection_in, connection_out);
        let position = self.map.get(&connect_hash);
        if position.is_none(){
            return None;
        }
        return self.connect_genes.get_mut(*position.unwrap());
    }
    pub fn contains_by_hash(&self, connect_hash: u64 ) -> bool{
        
        self.map.contains_key(&connect_hash)
    }
    pub fn get_by_hash(&self, connect_hash: u64 ) -> Option<&ConnectGene>{
        
        let position = self.map.get(&connect_hash);
        if position.is_none(){
            return None;
        }
        return self.connect_genes.get(*position.unwrap());
    }
    pub fn get_mut_unchecked(&mut self, connection_in: i32, connection_out:i32 ) -> &mut ConnectGene{
        
        let connect_hash = ConnectGene::compute_hash(connection_in, connection_out);
        let position = self.map.get(&connect_hash);
        return self.connect_genes.get_mut(*position.unwrap()).unwrap();
    }
    pub fn to_vec(&self) -> &Vec<ConnectGene>{
        
        self.connect_genes.as_ref()
    }
    pub fn from_vec(from: &Vec<ConnectGene>) -> Self{
        
        let mut new = Self::new(from.len());
        for connect_gene in from{
            new.add(connect_gene.to_owned());
        }
        new
    }
    pub fn new(capacity: usize) -> Self{
        
        Self {
            connect_genes: Vec::with_capacity(capacity),
            map: HashMap::with_capacity_and_hasher(capacity, BuildNoHashHasher::default())
        }
    }
    pub fn iter(&self) -> Iter<ConnectGene> {
        
        self.connect_genes.iter()
    }
    pub fn iter_mut(&mut self) -> IterMut<ConnectGene> {
        
        self.connect_genes.iter_mut()
    }
    pub fn len(&self) -> usize{
        
        self.connect_genes.len()
    }
    pub fn clear(&mut self){
        
        self.connect_genes.clear();
        self.map.clear();
    }
    pub fn get_random_connect_gene(&self) -> Option<&ConnectGene> {
        
        let len = self.connect_genes.len();
        if len == 0{
            return None;
        }
        let random_node_index = if len > 1 { Random::gen_range_usize(0, len-1) } else { 0 };
        Some(&self.connect_genes[random_node_index])
    }
    fn recompute_indexes(&mut self){
        
        self.map.clear();
        for i in 0..self.connect_genes.len(){
            let n = &self.connect_genes[i];
            self.map.insert(n.connection_hash, i);
        }
    }
}