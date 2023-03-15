use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::{node_kind::NodeKind, activation_functions::ActivationFunction, common::NodePosition};
use super::NeatFloat;
pub trait NetworkDefinition{
    fn get_network_identifier(&self) -> Uuid;
    fn get_node(&self, node_identity: i32) -> NetworkDefinitionNode;
    fn get_all_nodes(&self) -> Vec<NetworkDefinitionNode>;
    fn get_nodes_len(&self) -> u32;
    fn get_all_connections(&self) -> Vec<NetworkDefinitionConnection>;
    fn get_feed_connections_for_node(&self, node_identity: i32) -> Vec<NetworkDefinitionConnection>;
    fn get_output_nodes_count(&self) -> u32;
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct NetworkDefinitionNode{
    pub node_position: usize,
    pub identity: i32,
    pub kind: NodeKind,
    pub activation_function: ActivationFunction,
    pub bias: NeatFloat,
    pub input_multiplier: NeatFloat,
    pub position: Option<NodePosition>
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct NetworkDefinitionConnection{
    pub connection_in: i32,
    pub connection_out: i32,
    pub is_enabled: bool,
    pub is_recurrent: bool,
    pub weight: NeatFloat
}

