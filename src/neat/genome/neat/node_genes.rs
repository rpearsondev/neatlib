use std::slice::{Iter, IterMut};
use hashbrown::HashMap;
use nohash_hasher::BuildNoHashHasher;
use serde::{Serialize, Deserialize};
use crate::{node_kind::NodeKind, common::random::Random};

use super::node_gene::{NodeGene};

#[derive(Serialize, Deserialize)]
#[derive(Clone, Default, Debug)]
pub struct NodeGenes {
    node_genes: Vec<NodeGene>,
    map: HashMap<i32, usize, nohash_hasher::BuildNoHashHasher<i32>>
}
impl NodeGenes{
    pub fn from_vec(nodes: &Vec<NodeGene>) -> NodeGenes {
        let mut node_genes = Self::new(nodes.len());
        for n in nodes{
            node_genes.add(n.to_owned())
        }
        node_genes
    }
    pub fn new(capacity: usize) -> Self{
        NodeGenes{
            node_genes: Vec::with_capacity(capacity),
            map:  HashMap::with_capacity_and_hasher(capacity, BuildNoHashHasher::default())
        }
    }
    pub fn iter(&self) -> Iter<NodeGene> {
        self.node_genes.iter()
    }
    pub fn iter_mut(&mut self) -> IterMut<NodeGene> {
        self.node_genes.iter_mut()
    }
    pub fn get_node_numbers(&self) -> Vec<i32>{
        self.node_genes.iter().map(|n| n.number).collect::<Vec<i32>>()
    }
    pub fn has_node(&self, node_number: &i32) -> bool{
        self.map.contains_key(node_number)
    }
    pub fn add(&mut self, node: NodeGene){
        self.map.insert(node.number, self.node_genes.len());
        self.node_genes.push(node);
    }
    pub fn delete(&mut self, node_id: i32){
        let position = self.map.get(&node_id);
        if position.is_some(){
            let pos = *position.unwrap();
            self.node_genes.remove(pos);
            self.recompute_indexes();
        }
    }
    pub fn get(&self, node_id: i32) -> &NodeGene{
        let position = self.map.get(&node_id);
        return &self.node_genes.get(*position.unwrap()).unwrap();
    }
    pub fn get_with_position(&self, node_id: i32) -> (&NodeGene, usize){
        let position = self.map.get(&node_id);
        let pos = *position.unwrap();
        return (&self.node_genes.get(pos).unwrap(), pos);
    }
    pub fn get_opt(&self, node_id: i32) -> Option<&NodeGene>{
        let position = self.map.get(&node_id);
        if position.is_none(){
            return None;
        }
        return self.node_genes.get(*position.unwrap());
    }
    pub fn get_mut_unchecked(&mut self, node_id: i32) -> &mut NodeGene{
        let position = self.map.get(&node_id);
        return self.node_genes.get_mut(*position.unwrap()).unwrap();
    }
    pub fn get_mut(&mut self, node_id: i32) -> Option<&mut NodeGene>{
        let position = self.map.get(&node_id);
        if position.is_none(){
            return None;
        }
        return self.node_genes.get_mut(*position.unwrap());
    }
    pub fn len(&self) -> usize{
        self.node_genes.len()
    }
    pub fn get_random_hidden_node_number(&self) -> Option<i32> {
        self.get_random_node_number(|n| n.kind == NodeKind::Hidden)
    }
    pub fn get_random_node_number<F>(&self, filter: F) -> Option<i32>  where F: FnMut(&&NodeGene) -> bool {
        let nodes = self.node_genes.iter()
        .filter(filter)
        .collect::<Vec<&NodeGene>>();

        if nodes.len() == 0 {
            return None;
        }
        
        let len = nodes.len();
        let random_node_index = if len > 1 { Random::gen_range_usize(0, len) } else { 0 };
        Some(nodes[random_node_index].number)
    }
    fn recompute_indexes(&mut self){
        self.map.clear();
        for i in 0..self.node_genes.len(){
            let n = &self.node_genes[i];
            self.map.insert(n.number, i);
        }
    }
}