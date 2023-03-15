use bevy::prelude::Vec2;
use serde::{Deserialize, Serialize};
use crate::{activation_functions::ActivationFunction, neat::{trainer::node_conf::NodeConf, genome::neat::node_gene::NodeGene}, common::NeatFloat};
use super::{substrate_coordinate_scheme::SubstrateCoordinateScheme, substrate::Substrate, substrate_type::SubstrateType, substrate_geometric_organization::SubstrateGeometricOrganization, substrate_set_connection_mode::SubstrateSetConnectionMode, substrate_set_cppn_mode::SubstrateSetCPPNMode, cppn_inputs::{CppnInputs, CppnInput}, substrate_node_id_indexer::SubstrateNodeIdIndexer, substrate_node::{SubstrateNode, SubstrateCubeCoordinates, SubstrateSubstrateCoordinates}};

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct SubstrateSet{
    pub substrates: Vec<Substrate>,
    pub coordinate_scheme: SubstrateCoordinateScheme,
    pub geometric_organization: SubstrateGeometricOrganization,
    pub connection_mode: SubstrateSetConnectionMode,
    pub cppn_mode: SubstrateSetCPPNMode,
    pub activation_function: ActivationFunction,
    pub minimum_connection_weight: NeatFloat,
    pub substrate_nodes: Vec<SubstrateNode>,
    pub cppn_inputs: CppnInputs,
    pub node_id_indexer: SubstrateNodeIdIndexer,
}
impl SubstrateSet{
    pub fn new(
        coordinate_scheme: SubstrateCoordinateScheme,
        geometric_organization: SubstrateGeometricOrganization,
        activation_function: ActivationFunction,
        connection_mode: SubstrateSetConnectionMode,
        cppn_mode: SubstrateSetCPPNMode,
        input_substrate: Substrate, 
        hidden_substrates: Vec<Substrate>,
        output_substrate: Substrate        
    ) -> Self{
        if input_substrate.substrate_type != SubstrateType::Input{
            panic!("input substrate must be of type Input");
        }
   
        let mut substrates = vec![input_substrate];

        for hidden in hidden_substrates{
            if hidden.substrate_type != SubstrateType::Hidden{
                panic!("hidden substrate must be of type Hidden");
            }
            substrates.push(hidden);
        }

        if output_substrate.substrate_type != SubstrateType::Output{
            panic!("output substrate must be of type Input");
        }
        substrates.push(output_substrate);

        match connection_mode{
            SubstrateSetConnectionMode::Forward => {
                let len_of_substrates = substrates.len();
                for i in 0..len_of_substrates{
                    let mut substrate = &mut substrates[i];
                    substrate.connected_to_layers = (i+1..len_of_substrates).collect::<Vec<usize>>();
                    substrate.coordinate_scheme = coordinate_scheme;
                }
            }
            SubstrateSetConnectionMode::ForwardOne => {
                let len_of_substrates = substrates.len();
                for i in 0..len_of_substrates{
                    let mut substrate = &mut substrates[i];
                    substrate.connected_to_layers = vec![i+1];
                    substrate.coordinate_scheme = coordinate_scheme;
                }
            }
        }

        Self{
            coordinate_scheme,
            geometric_organization,
            connection_mode,
            substrates,
            activation_function,
            cppn_mode,
            minimum_connection_weight: 0.3,
            node_id_indexer: SubstrateNodeIdIndexer::new(),
            cppn_inputs: CppnInputs::default(),
            substrate_nodes: vec![]
        }
        .calculate_substrate_nodes()
        .create_cppn_inputs()
    }

    pub fn calculate_substrate_nodes(mut self) -> Self{
        
        let mut results: Vec<SubstrateNode> = Vec::new();
        for substrate_index in 0..self.substrates.len(){
            let substrate = &self.substrates[substrate_index];
            let node_positions = substrate.get_node_positions();
            for node_position in node_positions {
                let node_id = self.node_id_indexer.get_id((node_position[0], node_position[1], substrate_index as i32));
                results.push(
                    SubstrateNode { 
                        node_id: node_id, 
                        node_type: substrate.substrate_type, 
                        activation: self.activation_function, 
                        cube_coordinates: SubstrateCubeCoordinates { x: 0.0, y: 0.0, z: 0.0, angle_from_center:0.0, distance_from_center:0.0 }, 
                        substrate_coordinates: SubstrateSubstrateCoordinates { x: node_position[0], y: node_position[1], z: substrate_index as i32 } 
                    }
                );
            }
        }
        Self::resolve_cube_coordinates(&mut results);
        self.substrate_nodes = results;
        self
    }

    fn create_cppn_inputs(mut self) -> Self {
        
        let mut results: Vec<CppnInput> = Vec::new();
        let len_of_substrates = self.substrates.len();
        for substrate_index in 0.. len_of_substrates{
            let nodes_in_substrate = &self.substrate_nodes.iter()
            .filter(|n| n.substrate_coordinates.z == substrate_index as i32)
            .collect::<Vec<&SubstrateNode>>();

            let substrate = &self.substrates[substrate_index];
            for forward_substrate_index in &substrate.connected_to_layers{
                let nodes_in_forward_substrate = &self.substrate_nodes.iter()
                .filter(|n| n.substrate_coordinates.z == *forward_substrate_index as i32)
                .collect::<Vec<&SubstrateNode>>();
                
                for substrate_node in nodes_in_substrate{
                    for forward_node in nodes_in_forward_substrate{
                        results.push(CppnInput{
                            in_node: (*substrate_node).clone(),
                            out_node: (*forward_node).clone()
                        });
                    }
                }
            }
        }

        let expected_number_of_cppn_outputs: u32 =  match self.cppn_mode {
            SubstrateSetCPPNMode::XyzAngleDistanceToXyzAngleDistance => 2 // 1 weight and 1 target bias,
        };

        let expected_number_of_cppn_inputs: u32 =  match self.cppn_mode {
            SubstrateSetCPPNMode::XyzAngleDistanceToXyzAngleDistance => 10 
        };

        self.cppn_inputs = CppnInputs{
            cppn_mode: self.cppn_mode.clone(),
            inputs: results,
            expected_number_of_cppn_outputs,
            expected_number_of_cppn_inputs
        };

        self
    }
    fn resolve_cube_coordinates(nodes: &mut Vec<SubstrateNode>){
        
        let max_x = nodes.iter().map(|n| n.substrate_coordinates.x).max().unwrap_or_default();
        let min_x = nodes.iter().map(|n| n.substrate_coordinates.x).min().unwrap_or_default();
        let max_x_in_any_direction = i32::max(i32::max(i32::abs(max_x), i32::abs(min_x)), 1);

        let max_y = nodes.iter().map(|n| n.substrate_coordinates.y).max().unwrap_or_default();
        let min_y = nodes.iter().map(|n| n.substrate_coordinates.y).min().unwrap_or_default();
        let max_y_in_any_direction = i32::max(i32::max(i32::abs(max_y), i32::abs(min_y)), 1);

        let max_z = nodes.iter().map(|n| n.substrate_coordinates.z).max().unwrap_or_default();
        let min_z = nodes.iter().map(|n| n.substrate_coordinates.z).min().unwrap_or_default();
        let max_z_in_any_direction = i32::max(i32::max(i32::abs(max_z), i32::abs(min_z)), 1);
     
        for node in nodes{
            let x = node.substrate_coordinates.x as NeatFloat / max_x_in_any_direction as NeatFloat;
            let y = node.substrate_coordinates.y as NeatFloat / max_y_in_any_direction as NeatFloat;
            let center = Vec2{x: 10.0, y: 10.0};
            let node_vec  = Vec2{x: x+10.0, y: y+10.0};
            
            node.cube_coordinates= SubstrateCubeCoordinates { 
                x: x, 
                y: y, 
                z: node.substrate_coordinates.z as NeatFloat / max_z_in_any_direction as NeatFloat,  
                distance_from_center: node_vec.distance(center),
                angle_from_center:node_vec.angle_between(center)
            };
        }
    }
    pub fn get_cpp_node_conf(&self) ->  Box<Vec<NodeGene>>{
        NodeConf::simple(self.cppn_inputs.expected_number_of_cppn_inputs, self.cppn_inputs.expected_number_of_cppn_outputs)
    }
}

#[cfg(test)]
mod tests{
    use crate::{hyperneat::substrate::{substrate_coordinate_scheme::SubstrateCoordinateScheme, substrate_geometric_organization::SubstrateGeometricOrganization, substrate::Substrate, substrate_type::SubstrateType, substrate_set_connection_mode::SubstrateSetConnectionMode, substrate_set_cppn_mode::SubstrateSetCPPNMode}, activation_functions::ActivationFunction};
    use super::SubstrateSet;

    #[test]
    fn get_cppn_inputs_output_for_xyz() {
        let substrate_set = SubstrateSet::new(
            SubstrateCoordinateScheme::CenterOut, 
            SubstrateGeometricOrganization::Sandwich, 
            ActivationFunction::BIPOLAR_SIGMOID,
            SubstrateSetConnectionMode::Forward, 
            SubstrateSetCPPNMode::XyzAngleDistanceToXyzAngleDistance,
            Substrate::new(SubstrateType::Input, 1),
            vec![
                Substrate::new(SubstrateType::Hidden, 1), 
            ],
            Substrate::new(SubstrateType::Output, 1)); 
            let inputs = substrate_set.cppn_inputs;
            print!("{:?}", inputs);
            assert_eq!(inputs.inputs.len(), 3);
            assert_eq!(inputs.expected_number_of_cppn_outputs, 2)
    }

    #[test]
    fn get_cppn_inputs_output_for_xyz_multiple_nodes() {
        let substrate_set = SubstrateSet::new(
            SubstrateCoordinateScheme::CenterOut, 
            SubstrateGeometricOrganization::Sandwich, 
            ActivationFunction::BIPOLAR_SIGMOID,
            SubstrateSetConnectionMode::Forward, 
            SubstrateSetCPPNMode::XyzAngleDistanceToXyzAngleDistance,
            Substrate::new(SubstrateType::Input, 2),
            vec![
                Substrate::new(SubstrateType::Hidden, 2), 
            ],
            Substrate::new(SubstrateType::Output, 2)); 
            let inputs = substrate_set.cppn_inputs;
            assert_eq!(inputs.inputs.len(), 12); //3^2
            assert_eq!(inputs.expected_number_of_cppn_outputs, 2)
    }
}