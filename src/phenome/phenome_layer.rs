use crate::common::{network_definition::{NetworkDefinitionNode, NetworkDefinition}, NeatFloat};

use super::{phenome_layer_node::PhenomeLayerNode, node_index_lookup::NodePositionLookup};

#[derive(Debug, Clone)]
pub struct PhenomeLayer{
    pub nodes: Vec<PhenomeLayerNode>
}

impl PhenomeLayer{
pub fn new<TSchema: NetworkDefinition>(nodes: &Vec<NetworkDefinitionNode>, schema: &TSchema, node_index_lookup: &NodePositionLookup) -> Self 
{
    
    let mut phenotype_nodes: Vec<PhenomeLayerNode> = Vec::with_capacity(nodes.len());
    for n in nodes{
        phenotype_nodes.push(PhenomeLayerNode::new(n, schema, node_index_lookup));
    }
    PhenomeLayer {
        nodes: phenotype_nodes
    }
}
pub fn activate_sensors(&self, inputs: &Vec<NeatFloat>, node_results: &mut Vec<NeatFloat>, sensor_locations: &Vec<usize>){
    
    let mut c = 0 as usize;
    for i in inputs{
        node_results[sensor_locations[c]] += *i;
        c += 1;
    }
}
pub fn activate_layer(&self, node_results: &mut Vec<NeatFloat>){
    
    let len = self.nodes.len();
    for i in 0..len{
        self.nodes[i].activate(node_results);
    }
}
}
