// use neatlib::{node_kind::NodeKind, common::{NeatFloat, network_definition::NetworkDefinition, network_definition_node_layer_resolver::{NetworkDefinitionNodeLayerResolver, ResolvedNodeLayers}}, cpu_phenome::node_index_lookup::NodePositionLookup};
// use rayon::prelude::*;

// use super::node_feed_connections_index::NodeFeedConnectionsLookup;

// #[derive(Debug)]
// pub struct CombinedNetwork{
//     pub state: Vec<f32>,
//     pub layers: Vec<CombinedNetworkLayer>,
//     pub nodes: Vec<Node>,
//     pub connections: Vec<Connection>,
//     pub network_descriptors: Vec<NetworkDescriptor>,
// }

// #[derive(Debug)]
// pub struct CombinedNetworkLayer{
//     pub start_index: usize,
//     pub length: usize
// }

// #[derive(Clone)]
// pub struct NetworkWithSensorValues<T> where T:NetworkDefinition + Sized + Sync + Send + Clone{
//     pub network: T,
//     pub sensor_values: Vec<Vec<NeatFloat>>
// }

// impl CombinedNetwork{
//     pub fn from_networks<T>(networks: Vec<NetworkWithSensorValues<T>>) -> Self where T:NetworkDefinition + Sized + Sync + Send + Clone{
        
//         let mut networks_with_computed_layers: Vec<(NetworkWithSensorValues<T>, ResolvedNodeLayers, NodePositionLookup, NodeFeedConnectionsLookup)> = Vec::with_capacity(networks.len());
//         let _par_iter = networks.into_par_iter().map(|network_with_sensor_values|{
//             let nodes = &network_with_sensor_values.network.get_all_nodes();
//             let node_position_lookup = NodePositionLookup::new_from_nodes(nodes);
//             let layers = NetworkDefinitionNodeLayerResolver::get_node_layers(&network_with_sensor_values.network, true);
//             let node_feed_connection_index  = NodeFeedConnectionsLookup::new_from_nodes(nodes, &network_with_sensor_values.network, &node_position_lookup);
//             (network_with_sensor_values, layers, node_position_lookup, node_feed_connection_index)
//         })
//         .collect_into_vec(&mut networks_with_computed_layers);

//         let mut total_state_length = 0;
//         let mut total_connections = 0;
//         let mut total_nodes = 0;
//         let mut total_networks = 0;
//         let mut max_network_depth = 0;
//         for (n, resolved_nodes_layers, _, _) in &networks_with_computed_layers{
            
//             total_state_length += resolved_nodes_layers.total_nodes as usize * n.sensor_values.len();
//             total_connections += resolved_nodes_layers.total_connections as usize * n.sensor_values.len();
//             total_nodes += resolved_nodes_layers.total_nodes as usize * n.sensor_values.len();
//             total_networks += 1 * n.sensor_values.len();
//             if resolved_nodes_layers.layers.len() > max_network_depth{
//                 max_network_depth = resolved_nodes_layers.layers.len()
//             }
//         }

//         let mut state: Vec<f32> = Vec::with_capacity(total_state_length);
//         state.resize(total_state_length, 0.0);
//         let mut connections: Vec<Connection> = Vec::with_capacity(total_connections as usize);
//         let mut nodes: Vec<Node> = Vec::with_capacity(total_nodes as usize);
//         let mut network_descriptors: Vec<NetworkDescriptor> = Vec::with_capacity(total_networks);
//         let mut layers: Vec<CombinedNetworkLayer> = Vec::with_capacity(max_network_depth);
//         let mut current_end_of_combined_state = 0;
//         let mut temp_nodes: Vec<Vec<Node>> = Vec::new();
//         temp_nodes.resize_with(max_network_depth, ||{Vec::new()});

//         for (network_with_sensor_values, resolved_node_layers, _, connections_index) in &networks_with_computed_layers{
            
//             let network = &network_with_sensor_values.network;
//             for sensor_values in &network_with_sensor_values.sensor_values{
//                 let mut sensor_index = 0;
//                 let mut layer_index = 1 as usize;
//                 let mut output_addresses: Vec<usize> = Vec::new();
//                 for layer in &resolved_node_layers.layers{
//                     let temp_nodes_layer = &mut temp_nodes[layer_index-1];

//                     for layer_node in layer{
//                         let node_connections = connections_index.get_feed_connections_for_node(layer_node.node_position);

//                         let mut node_connection_for_combined = node_connections.iter()
//                         .map(|c| Connection{
//                             from_state_index: c.in_node_position as u32 + current_end_of_combined_state as u32,
//                             weight: c.weight
//                         }).collect::<Vec<_>>();
//                         let connections_from = connections.len();
//                         let connections_to = connections_from + node_connection_for_combined.len();
        
//                         connections.append(&mut node_connection_for_combined);
        
//                         let node_address_in_combined = layer_node.node_position + current_end_of_combined_state as usize;

//                         if layer_node.kind == NodeKind::Sensor{
//                             state[node_address_in_combined] = sensor_values[sensor_index];
//                             sensor_index += 1;
//                         }
        
//                         if layer_node.kind == NodeKind::Output{
//                             output_addresses.push(node_address_in_combined);
//                         }

//                         temp_nodes_layer.push(Node{
//                             address: node_address_in_combined as u32,
//                             identity: layer_node.identity,
//                             activation: layer_node.activation_function.bits(),
//                             bias: layer_node.bias,
//                             connections_index_from: connections_from as u32,
//                             connections_index_to_exc: connections_to as u32,
//                             sum: 0.0
//                         });
//                     }

//                     layer_index += 1;
//                 }

//                 let mut outputs: Vec<NeatFloat> = Vec::new();
//                 outputs.resize(resolved_node_layers.total_outputs.unwrap() as usize, 0.0 as NeatFloat);
//                 network_descriptors.push(NetworkDescriptor{
//                     network_identifier: network.get_network_identifier(),
//                     outputs: outputs,
//                     output_addresses: output_addresses
//                 });

//                 current_end_of_combined_state += resolved_node_layers.total_nodes;
//             }
//         }

//         for layer in temp_nodes.iter_mut(){
//             layers.push(CombinedNetworkLayer{
//                start_index: nodes.len(),
//                length: layer.len() 
//             });
//             nodes.append(layer)
//         }

//         Self{
//             state,
//             connections: connections,
//             layers: layers,
//             nodes: nodes,
//             network_descriptors: network_descriptors,
//         }
//     }
// }


// #[repr(C)]
// #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
// pub struct Node{
//     pub identity: i32,
//     pub address: u32,
//     pub activation: u32,
//     pub bias: f32,
//     pub sum: f32,
//     pub connections_index_from: u32,
//     pub connections_index_to_exc: u32
// }

// #[repr(C)]
// #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
// pub struct Connection{
//     pub from_state_index: u32,    
//     pub weight: f32
// }

// #[derive(Debug)]
// pub struct NetworkDescriptor{
//     pub network_identifier: uuid::Uuid,
//     pub output_addresses: Vec<usize>,
//     pub outputs: Vec<NeatFloat>
// }

// #[cfg(test)]
// pub mod tests{
//     use neatlib::{neat::{trainer::{configuration::Configuration, run_context::RunContext}, genome::{neat::{node_gene::NodeGene, NeatGenome, connect_gene::ConnectGene}, genome::Genome}}, node_kind::NodeKind, activation_functions::ActivationFunction};

//     use crate::gpu::combined_network::{NetworkWithSensorValues, CombinedNetwork};

//     #[test]
//     pub fn add_simple_network_correct_nodes_and_connection(){        
//         let configuration  = Configuration::neat(
//             Box::new(vec![
//                 NodeGene::new(1, NodeKind::Sensor),
//                 NodeGene::new_hidden(2, ActivationFunction::RELU),
//                 NodeGene::new(3, NodeKind::Output)
//             ]), 0.0);
        
//             let mut run_context =  RunContext::new(3, 0);
//             let mut network_1 = NeatGenome::minimal(&configuration, &mut run_context);
//             network_1.genes.connect.clear();
//             network_1.genes.connect.add(ConnectGene::new_with_weight(1, 2, 1.0 ,true));
//             network_1.genes.connect.add(ConnectGene::new_with_weight(2, 3, 1.0, true));

//             let mut networks: Vec<NetworkWithSensorValues<NeatGenome>> = Vec::new();
//             for _ in 0..1 {
//                 let n = network_1.clone();
//                 networks.push(NetworkWithSensorValues{
//                     network: n,
//                     sensor_values: vec![vec![0.0], vec![0.0], vec![0.0]]
//                 });
//             }

//             let combined_network = CombinedNetwork::from_networks(networks);

//             println!("{:?}", combined_network);
//     }
// }