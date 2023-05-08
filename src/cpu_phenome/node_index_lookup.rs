use hashbrown::HashMap;
use nohash_hasher::BuildNoHashHasher;

use crate::common::network_definition::{NetworkDefinition, NetworkDefinitionNode};

pub struct NodePositionLookup{
    lookup: HashMap<i32, usize, nohash_hasher::BuildNoHashHasher<i32>>
}
impl NodePositionLookup {
    pub fn new<S>(schema: &S) -> Self where S: NetworkDefinition{
        
        let all_nodes = schema.get_all_nodes();
        Self::new_from_nodes(&all_nodes)
    }
    pub fn new_from_nodes(all_nodes: &Vec<NetworkDefinitionNode>) -> Self{
        
        let mut lookup:  HashMap<i32, usize, nohash_hasher::BuildNoHashHasher<i32>> = HashMap::with_capacity_and_hasher(all_nodes.len(), BuildNoHashHasher::default());
        for n in all_nodes{
            lookup.insert(n.identity, n.node_position);
        }
        NodePositionLookup{
            lookup: lookup
        }
    }
    pub fn get_node_position(&self, index: i32) -> usize{
        *self.lookup.get_key_value(&index).unwrap().1
    }
    pub fn len(&self) -> usize{
        self.lookup.len()
    }
}
