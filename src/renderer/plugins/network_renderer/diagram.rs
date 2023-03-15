use std::sync::Arc;

use bevy::prelude::*;
use colorgrad::Color as ColorGrad;

use crate::{common::{network_definition::NetworkDefinition, network_definition_node_layer_resolver::NetworkDefinitionNodeLayerResolver}, node_kind::NodeKind};

const DISTANCE_BETWEEN_NODES_X: f32 = 0.5;
const DISTANCE_BETWEEN_NODES_Y: f32 = 0.5;
const SENSOR_OUTPUT_Z: f32 = 0.3;
const NODE_SIZE: f32 = 0.1;

#[derive(Clone)]
pub struct DiagramNode{
    pub identity: i32,
    pub color: Color,
    pub text: String,
    pub position: Vec3,
    pub size: f32
}
pub struct DiagramConnection{
    pub from_position: Vec3,
    pub to_position: Vec3,
    pub color: Color
}
pub struct Diagram{
    pub nodes: Vec<DiagramNode>,
    pub connections: Vec<DiagramConnection>,
}
impl Diagram{
    pub fn from<T>(network: &Arc<T>) -> Self where T:NetworkDefinition{
        let mut diagram_nodes :Vec<DiagramNode> = Vec::new();

        let resolved_node_layers = NetworkDefinitionNodeLayerResolver::get_node_layers(network.as_ref(), false);
        let mut layer_index = 0;
        
        let max_nodes_in_any_layer = resolved_node_layers.layers.iter().map(|layer| layer.len()).max().unwrap_or_default();
        if max_nodes_in_any_layer == 0 {
            return Self{
                connections: vec![],
                nodes: vec![]
            }
        }
        let scale_factor = Diagram::get_scale_factor(resolved_node_layers.layers.len(), max_nodes_in_any_layer);
        let x_diagram_offset = -((max_nodes_in_any_layer) as f32 * Diagram::scale(scale_factor,DISTANCE_BETWEEN_NODES_X)) / 2.0;
        let y_diagram_offset = -((resolved_node_layers.layers.len()) as f32 * Diagram::scale(scale_factor,DISTANCE_BETWEEN_NODES_Y)) / 2.0;
        while layer_index < resolved_node_layers.layers.len(){
            let mut node_index = 0;
            let mut layer = resolved_node_layers.layers[layer_index].clone();
            layer.sort_by(|a,b| a.identity.cmp(&b.identity));
            let nodes_in_layer = layer.len();

            let x_offset = x_diagram_offset + ((max_nodes_in_any_layer - nodes_in_layer) as f32 * Diagram::scale(scale_factor,DISTANCE_BETWEEN_NODES_X)) / 2.0;
            
            while node_index < layer.len(){
                let node = &layer[node_index];
                let mut color = Color::GRAY;
                let mut z_offset: f32 = 0.0;
                let mut size = Diagram::scale(scale_factor, NODE_SIZE);
                if node.kind == NodeKind::Sensor{
                    color = Color::PINK;
                    z_offset = Diagram::scale(scale_factor,SENSOR_OUTPUT_Z);
                    size = size * 1.5;
                }

                if node.kind == NodeKind::Output{
                    color = Color::BLUE;
                    z_offset = Diagram::scale(scale_factor,SENSOR_OUTPUT_Z);
                    size = size * 1.5;
                }

                diagram_nodes.push(DiagramNode {
                    identity: node.identity,
                    color: color,
                    text: "".to_string(),
                    size: size,
                    position: Vec3 { x: (node_index as f32 * Diagram::scale(scale_factor,DISTANCE_BETWEEN_NODES_X)) + x_offset, y: y_diagram_offset + (layer_index as f32 * Diagram::scale(scale_factor,DISTANCE_BETWEEN_NODES_Y)), z: z_offset }
                });
                
                node_index +=1;
            }
            layer_index +=1;
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

        let mut diagram_connections :Vec<DiagramConnection> = Vec::new();
        let nodes_clone = diagram_nodes.clone();
        for diagram_node in nodes_clone{
            let feed_connections = network.get_feed_connections_for_node(diagram_node.identity);
            for connection in feed_connections{
                let n = diagram_nodes.iter().find(|n| n.identity == connection.connection_in);
                if n.is_some(){
                    let from_node = n.unwrap();
                    let color: ColorGrad;
                    if connection.weight > 0.0{
                        color = gradient_positive.at(connection.weight as f64);
                    }else{
                        color = gradient_negative.at(-connection.weight as f64);
                    }
                    diagram_connections.push(DiagramConnection { from_position: from_node.position, to_position: diagram_node.position, color: Color::rgba(color.r as f32, color.g as f32, color.b as f32, color.a as f32) })
                }
            }
        }

        Self{
            nodes: diagram_nodes,
            connections: diagram_connections
        }
    }
    fn scale(factor: f32, value: f32) -> f32{
        value * factor
    }
    fn get_scale_factor(max_layers: usize, max_layer_width: usize) -> f32{
        let max = usize::max(max_layers, max_layer_width);
        1.0 / (max as f32 / 5.0 as f32)
    }
}
