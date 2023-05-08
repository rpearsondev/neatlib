// use neatlib::{phenome::node_index_lookup::NodePositionLookup, common::{NeatFloat, network_definition::{NetworkDefinitionNode, NetworkDefinition}}};


// pub struct NetworkFeedConnection{
//     pub weight:NeatFloat,
//     pub in_node_position: usize
// }

// pub struct NodeFeedConnectionsLookup{
//     lookup: Vec<Vec<NetworkFeedConnection>>
// }

// impl NodeFeedConnectionsLookup {
//     pub fn new_from_nodes<TSchema>(all_nodes: &Vec<NetworkDefinitionNode>, schema: &TSchema, node_position_lookup: &NodePositionLookup) -> Self where TSchema:NetworkDefinition{
//         let mut lookup: Vec<Vec<NetworkFeedConnection>> = Vec::new();
//         lookup.resize_with(all_nodes.len(), ||{Vec::new()});
//         for node in all_nodes{
//             let feed_connections = schema.get_feed_connections_for_node(node.identity);
//             let node_feed_connections = feed_connections.iter().map(|c|{
//                 NetworkFeedConnection{
//                     weight: c.weight,
//                     in_node_position: node_position_lookup.get_node_position(c.connection_in)
//                 }
//             }).collect::<Vec<NetworkFeedConnection>>();
//             lookup[node.node_position] = node_feed_connections;
//         }
//         Self { lookup }
//     }
//     pub fn get_feed_connections_for_node(&self, node_position: usize) -> &Vec<NetworkFeedConnection>{
//         &self.lookup[node_position]
//     }
// }