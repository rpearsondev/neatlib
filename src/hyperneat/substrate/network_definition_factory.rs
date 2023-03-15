use rayon::prelude::IntoParallelRefIterator;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::{common::{network_definition::{NetworkDefinition, NetworkDefinitionNode, NetworkDefinitionConnection}, NeatFloat}, phenome::{Phenome}, node_kind::NodeKind};
use super::{substrate_set::SubstrateSet, cppn_inputs::CppnInputs, substrate_set_cppn_mode::SubstrateSetCPPNMode, substrate_node::SubstrateNode, substrate_type::SubstrateType};
use rayon::prelude::*;
pub struct NetworkDefinitionFactory;

impl NetworkDefinitionFactory{
    pub fn produce(cppn: Phenome, substrate_set: &SubstrateSet) -> XyzSubstrateNetworkDefinition{
        
        let cppn_inputs = &substrate_set.cppn_inputs;

        let network = match cppn_inputs.cppn_mode {
            SubstrateSetCPPNMode::XyzAngleDistanceToXyzAngleDistance =>    XyzSubstrateNetworkDefinition::new(Self::xyz_mapping(cppn, cppn_inputs, substrate_set))
        };
        network
    }

    pub fn xyz_mapping(cppn: Phenome, cppn_inputs:&CppnInputs, substrate_set: &SubstrateSet) -> XyzNetworkParameters{
        
        let mut xyz_network_parameters : Vec<XyzNetworkParameter> = Vec::new();

        cppn_inputs.inputs.par_iter()
            .map(|input|{
            let cppn_outputs = cppn.clone().activate(&vec![
                input.in_node.cube_coordinates.x,
                input.in_node.cube_coordinates.y,
                input.in_node.cube_coordinates.z,
                input.in_node.cube_coordinates.distance_from_center,
                input.in_node.cube_coordinates.angle_from_center,
                input.out_node.cube_coordinates.x,
                input.out_node.cube_coordinates.y,
                input.out_node.cube_coordinates.z,
                input.out_node.cube_coordinates.distance_from_center,
                input.out_node.cube_coordinates.angle_from_center,
            ]);

            let connection_weight = cppn_outputs[0];
            let output_bias = cppn_outputs[1];
            XyzNetworkParameter{
                    in_node: input.in_node.clone(),
                    out_node: input.out_node.clone(),
                    connection_weight: connection_weight,
                    out_bias: output_bias,
            }
        }).collect_into_vec(&mut xyz_network_parameters);

        let xyz_network_parameters = xyz_network_parameters
        .iter()
        .filter(|p| NeatFloat::abs(p.connection_weight) > substrate_set.minimum_connection_weight)
        .map(|c| *c)
        .collect::<Vec<XyzNetworkParameter>>();
      
        XyzNetworkParameters{
            id: cppn.id,
            connections: xyz_network_parameters,
            nodes: substrate_set.substrate_nodes.clone()
        }
        
    }
}

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct XyzNetworkParameter{
    pub in_node:SubstrateNode,
    pub out_node: SubstrateNode,
    pub connection_weight: NeatFloat,
    pub out_bias: NeatFloat,
}

#[derive(Clone, Debug)]
pub struct XyzNetworkParameters{
    pub id: Uuid,
    pub connections: Vec<XyzNetworkParameter>,
    pub nodes: Vec<SubstrateNode>
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct XyzSubstrateNetworkDefinition{
    pub id: uuid::Uuid,
    pub parameters: Vec<XyzNetworkParameter>,
    connections: Vec<NetworkDefinitionConnection>,
    node_connections:  Vec<Vec<NetworkDefinitionConnection>>,
    nodes: Vec<NetworkDefinitionNode>
}

impl XyzSubstrateNetworkDefinition{
    pub fn new(parameters: XyzNetworkParameters) -> Self{
        
        let mut nodes =  parameters.nodes.iter().enumerate().map(|(pos, n)| NetworkDefinitionNode{
            node_position: pos,
            identity: n.node_id,
            kind: get_node_kind(n.node_type),
            activation_function: n.activation,
            bias: 0.0,
            input_multiplier: 1.0,
            position: Some((n.cube_coordinates.x,n.cube_coordinates.x, n.cube_coordinates.z)),
        }).collect::<Vec<NetworkDefinitionNode>>();

        for i in 0..parameters.connections.len(){
            let connection = parameters.connections[i];
            let bias = connection.out_bias;
            let node_opt = nodes.iter_mut().find(|n| {n.identity == connection.out_node.node_id});
            if node_opt.is_some(){
                let node = node_opt.unwrap();
                node.bias = bias;
            }
        }

        let connections = parameters.connections.iter().map(|p| {
            return NetworkDefinitionConnection{
                connection_in: p.in_node.node_id,
                connection_out: p.out_node.node_id,
                is_enabled: true,
                is_recurrent: false,
                weight: p.connection_weight
            };
        }).collect::<Vec<NetworkDefinitionConnection>>();
        let mut node_connections: Vec<Vec<NetworkDefinitionConnection>> = Vec::new();
        for _ in 0..=nodes.len(){
            node_connections.push(Vec::new())
        }

        for connection in connections.iter(){
            node_connections[connection.connection_out as usize].push((*connection).clone());
        }

        Self { 
            id: parameters.id,
            nodes,
            connections,
            parameters: parameters.connections,
            node_connections
        }
    }
}

impl NetworkDefinition for XyzSubstrateNetworkDefinition{
    fn get_network_identifier(&self) -> uuid::Uuid {
        self.id
    }

    fn get_node(&self, node_identity: i32) -> NetworkDefinitionNode {
        self.nodes.iter().find(|n| n.identity == node_identity).unwrap().clone()
    }

    fn get_all_nodes(&self) -> Vec<NetworkDefinitionNode> {
        self.nodes.clone()
    }

    fn get_all_connections(&self) -> Vec<NetworkDefinitionConnection> {
       self.connections.clone()
    }

    fn get_feed_connections_for_node(&self, node_identity: i32) -> Vec<NetworkDefinitionConnection> {
        self.node_connections[node_identity as usize].clone()
    }

    fn get_nodes_len(&self) -> u32 {
        self.nodes.len() as u32
    }

    fn get_output_nodes_count(&self) -> u32 {
        self.nodes.iter().filter(|n| n.kind == NodeKind::Output).count() as u32
    }
}

fn get_node_kind(substrate_type: SubstrateType) -> NodeKind{
    
    let result = match substrate_type {
        SubstrateType::Input => NodeKind::Sensor,
        SubstrateType::Output => NodeKind::Output,
        SubstrateType::Hidden => NodeKind::Hidden,
    };
    result
}


#[cfg(test)]
mod tests{
    use crate::{hyperneat::substrate::{substrate_coordinate_scheme::SubstrateCoordinateScheme, substrate_geometric_organization::SubstrateGeometricOrganization, substrate::Substrate, substrate_type::SubstrateType, substrate_set_connection_mode::SubstrateSetConnectionMode, substrate_set_cppn_mode::SubstrateSetCPPNMode}, activation_functions::ActivationFunction, neat::{genome::{neat::{NeatGenome, mutation_mode::MutationMode}, genome::Genome}, trainer::{configuration::Configuration, run_context::RunContext}}, phenome::Phenome};
    use super::{SubstrateSet, NetworkDefinitionFactory};

    #[test]
    #[ignore = "Hyperneat is going to be re-developed to es-hyperneat"]
    fn get_cppn_inputs_output_for_each_layer() {
        let substrate_set = SubstrateSet::new(
            SubstrateCoordinateScheme::CenterOut, 
            SubstrateGeometricOrganization::Sandwich, 
            ActivationFunction::BIPOLAR_SIGMOID,
            SubstrateSetConnectionMode::Forward, 
            SubstrateSetCPPNMode::XyzAngleDistanceToXyzAngleDistance,
            Substrate::new(SubstrateType::Input, 4), 
            vec![
                //Substrate::new(SubstrateType::Hidden, 9), 
            ],
            Substrate::new(SubstrateType::Output, 9)); 
        
            let configuration  = Configuration::neat(substrate_set.get_cpp_node_conf(), 0.0)
                .mutation_node_available_activation_functions(ActivationFunction::for_cppn());
        
            let context = &mut RunContext::new(10, 0);
            let mut minimal_genome = NeatGenome::minimal(&configuration, context);
        
            for _ in 1..10{
                minimal_genome.mutate(&configuration, context, MutationMode::Steady)
            }

            let cppn = Phenome::from_network_schema(&minimal_genome);

            let hyperneat_network = NetworkDefinitionFactory::produce(cppn, &substrate_set);

            let net = Phenome::from_network_schema(&hyperneat_network);

            let results = net.activate(&vec![0.5, 0.5, 0.5, 0.5]);

            println!("{:?}", results);
    }
}