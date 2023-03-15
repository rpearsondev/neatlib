use crate::common::{network_definition::{NetworkDefinitionNode, NetworkDefinition}, NeatFloat};
use super::{phenome_layer_node_connection::PhenomeLayerNodeConnection, ActivationFunction, node_index_lookup::NodePositionLookup, activation_mapper::ActivationMapper};

#[derive(Debug, Clone)]
pub struct PhenomeLayerNode{
    pub array_index: usize,
    pub bias: NeatFloat,
    pub input_multiplier: NeatFloat,
    pub feed_connections: Vec<PhenomeLayerNodeConnection>,
    pub activation: ActivationFunction
}
impl PhenomeLayerNode{
    pub fn new<TSchema: NetworkDefinition>
    (node: &NetworkDefinitionNode, schema: &TSchema, node_index_lookup: &NodePositionLookup) -> Self
    {
        
        let connections = schema.get_feed_connections_for_node(node.identity)
        .iter()
        .map(|c|{
            PhenomeLayerNodeConnection{
                from_node_array_index: node_index_lookup.get_node_position(c.connection_in),
                weight: c.weight
            }
        }).collect::<Vec<PhenomeLayerNodeConnection>>();

        PhenomeLayerNode {
            activation: ActivationMapper::map(node.kind.clone(), node.activation_function.clone()),
            bias: node.bias,
            input_multiplier: node.input_multiplier,
            feed_connections: connections,
            array_index: node.node_position
        }
    }
    pub fn activate(&self, node_results: &mut Vec<NeatFloat>){
        let mut val = self.bias;
        for connection in &self.feed_connections{
            val += node_results[connection.from_node_array_index] * connection.weight;
        }
        let x = self.activation;
        node_results[self.array_index] = x(val, self.input_multiplier);
    }
}
