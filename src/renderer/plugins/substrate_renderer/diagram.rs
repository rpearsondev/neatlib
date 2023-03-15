use bevy::prelude::*;
use colorgrad::Color as ColorGrad;
use crate::{hyperneat::substrate::{substrate_set::SubstrateSet, substrate_type::SubstrateType, network_definition_factory::XyzNetworkParameter, substrate_node::SubstrateNode}};

const _DISTANCE_BETWEEN_NODES: f32 = 0.5;
const DISTANCE_BETWEEN_SUBSTRATES: f32 = 0.5;
const SUBSTRATE_WIDTH: f32 = 1.0;
const SUBSTRATE_MAX_WIDTH: f32 = 2.0;
const _NODE_SIZE: f32 = 0.05;

#[derive(Clone)]
pub struct DiagramSubstrateNode{
    pub id: i32,
    pub node_x: i32,
    pub node_y: i32,
    pub color: Color,
    pub position: Vec3,
    pub size: f32,
}
pub struct DiagramSubstrate{
    pub substrate_number: i32,
    pub color: Color,
    pub position: Vec3,
    pub width: f32
}

pub struct DiagramAnnotation{
    pub text: String,
    pub size: f32,
    pub position: Vec3,
    pub offset_x: f32,
    pub offset_y: f32,
}

pub struct DiagramConnection{
    pub from_position: Vec3,
    pub to_position: Vec3,
    pub color: Color,
    pub weight: f32
}

pub struct Diagram{
    pub nodes: Vec<DiagramSubstrateNode>,
    pub substrates: Vec<DiagramSubstrate>,
    pub annotations: Vec<DiagramAnnotation>,
    pub connections: Vec<DiagramConnection>
}
impl Diagram{
    pub fn from(substrate_set: &SubstrateSet, connections_opt: &Option<Vec<XyzNetworkParameter>>) -> Diagram{
        let mut nodes :Vec<DiagramSubstrateNode> = Vec::new();
        let mut substrates :Vec<DiagramSubstrate> = Vec::new();
        let mut annotations :Vec<DiagramAnnotation> = Vec::new();
        
        let number_of_substrates = substrate_set.substrates.len();
        for i in 0..number_of_substrates{
            let substrate = &substrate_set.substrates[i];
            let node_positions = substrate_set.substrate_nodes.iter()
            .filter(|n| n.substrate_coordinates.z == i as i32)
            .collect::<Vec<&SubstrateNode>>();
            let number_of_nodes = node_positions.len() as f32;
            let mut distance_between_nodes = SUBSTRATE_WIDTH / 10.0;
            let mut substrate_size = distance_between_nodes * (number_of_nodes as f32).sqrt();
            let mut node_annotation_size = 3.0;
            let mut node_size = SUBSTRATE_WIDTH / 50.0;
            let mut only_even_nodes = false;
            if substrate_size > SUBSTRATE_MAX_WIDTH{
                node_annotation_size = 2.0;
                only_even_nodes = true;
                substrate_size = SUBSTRATE_MAX_WIDTH;
                distance_between_nodes = substrate_size / (number_of_nodes as f32).sqrt();
                node_size = SUBSTRATE_MAX_WIDTH / 100.0
            }
            substrate_size+= 0.2;//add margin


            let mut color = Color::GRAY;
            if substrate.substrate_type == SubstrateType::Input{
                color = Color::PINK;
            }

            if substrate.substrate_type == SubstrateType::Output{
                color = Color::BLUE;
            }

            color.set_a(0.4);
            let substrate_position = Vec3 { x: 0.0, y: 0.0, z: -(i as f32 * DISTANCE_BETWEEN_SUBSTRATES) };
            substrates.push(DiagramSubstrate{
                color: color,
                substrate_number: i as i32 +1,
                position: substrate_position,
                width: substrate_size
            });

            annotations.push(DiagramAnnotation{
                text: format!("{} (total:{})", i, number_of_nodes),
                size: 4.0,
                position: substrate_position,
                offset_x: substrate_size / 2.0,
                offset_y: substrate_size / 2.0
            });

            
            for node in node_positions{
                let x = node.substrate_coordinates.x;
                let y = node.substrate_coordinates.y;
                let offset_x = x as f32 * distance_between_nodes;
                let offset_y = y as f32 * distance_between_nodes;
                let mut node_position = substrate_position;
                node_position.x = offset_x;
                node_position.y = offset_y;
                node_position.z = substrate_position.z;

                nodes.push(DiagramSubstrateNode{
                    id: node.node_id,
                    node_x: x,
                    node_y: y,
                    color: Color::PINK,
                    position: node_position,
                    size: node_size
                });

                if !only_even_nodes || y  % 2 == 0 { 
                    annotations.push(DiagramAnnotation{
                        text: format!("{},{}", x, y),
                        size: node_annotation_size,
                        position: substrate_position,
                        offset_x: -offset_y,
                        offset_y: offset_x,
                    });
                }
            }
        }


        let gradient_positive = colorgrad::CustomGradient::new()
        .colors(&[
            ColorGrad::from_rgba8(0, 0, 0, 255),
            ColorGrad::from_rgba8(0, 255, 0, 255),
        ])
        .build().unwrap();

        let gradient_negative = colorgrad::CustomGradient::new()
        .colors(&[
            ColorGrad::from_rgba8(0, 0, 0, 255),
            ColorGrad::from_rgba8(255, 0, 0, 255),
        ])
        .build().unwrap();

        let mut connections: Vec<DiagramConnection> = Vec::new();
        if connections_opt.is_some(){
            let network_connections = connections_opt.as_ref().unwrap();
            for network_connection in network_connections{
                
                let color: ColorGrad;
                if network_connection.connection_weight > 0.0{
                    color = gradient_positive.at(network_connection.connection_weight as f64);
                }else{
                    color = gradient_negative.at(-network_connection.connection_weight as f64);
                }

                let from_diagram_node = nodes.iter().find(|n| n.id == network_connection.in_node.node_id).unwrap();
                let to_diagram_node = nodes.iter().find(|n| n.id == network_connection.out_node.node_id).unwrap();

                connections.push(DiagramConnection {  
                    from_position: from_diagram_node.position,
                    to_position: to_diagram_node.position,
                    weight: network_connection.connection_weight,
                    color: Color::rgba(color.r as f32, color.g as f32, color.b as f32, color.a as f32) 
                })
            }
        }

        Self{
            nodes: nodes,
            substrates,
            annotations,
            connections
        }
    }
    fn _scale(factor: f32, value: f32) -> f32{
        value * factor
    }
    fn _get_scale_factor(max_layers: usize, max_layer_width: usize) -> f32{
        let max = usize::max(max_layers, max_layer_width);
        1.0 / (max as f32 / 5.0 as f32)
    }
}
