use std::{collections::hash_map::DefaultHasher, hash::{Hasher, Hash}};

use serde::{Serialize, Deserialize};

use crate::common::NeatFloat;

use super::NeatGenome;

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub enum Mutation {
    //connect_in, connect_out, new_value
    ConnectionChangeWeight(i32, i32 ,NeatFloat), 
    //connect_in, connect_out
    DeleteConnection(i32, i32),
    //node_id, bias
    AddNode(i32, f32),
    //node_id
    DeleteNode(i32),
    //connect_in, connect_out, initial_weight
    AddConnection(i32, i32, f32),
    //node_id, new_value
    NodeBiasChange(i32, f32),
    //node_id, new_value
    CppnInputMultiplierChange(i32, f32), 
}

impl Mutation{
    pub fn get_identity(mutation: Mutation) -> u64{
        let mut hasher = DefaultHasher::new();
        
        match mutation{
            Mutation::ConnectionChangeWeight(connection_in, connection_out, _) => {
                connection_in.hash(&mut hasher);
                connection_out.hash(&mut hasher);
            },
            Mutation::DeleteConnection(connection_in, connection_out) => {
                connection_in.hash(&mut hasher);
                connection_out.hash(&mut hasher);
            },
            Mutation::AddNode(id, _) => {
                id.hash(&mut hasher);
            },
            Mutation::DeleteNode(id) => {
                id.hash(&mut hasher);
            },
            Mutation::AddConnection(connection_in, connection_out, _) => {
                connection_in.hash(&mut hasher);
                connection_out.hash(&mut hasher);
            },
            Mutation::NodeBiasChange(id, _) => {
                id.hash(&mut hasher);
            },
            Mutation::CppnInputMultiplierChange(id, _) => {
                id.hash(&mut hasher);
            }
        }

        let connection_hash = hasher.finish();
        connection_hash
    }
    pub fn get_value(mutation: Mutation) -> Option<NeatFloat>{
        match mutation{
            Mutation::ConnectionChangeWeight(_, _, v) => {
                return Some(v);
            },
            Mutation::DeleteConnection(_, _) => {
                return None;
            },
            Mutation::AddNode(_, _) => {
                return None;
            },
            Mutation::DeleteNode(_) => {
                return None;
            },
            Mutation::AddConnection(_, _, v) => {
                return Some(v);
            },
            Mutation::NodeBiasChange(_, v) => {
                return Some(v);
            },
            Mutation::CppnInputMultiplierChange(_, v) => {
                return Some(v);
            }
        }
    }
    pub fn apply_optimization_to_genome(mutation: Mutation, genome: &mut NeatGenome){
        match mutation{
            Mutation::ConnectionChangeWeight(connection_in, connection_out, value) => {
                let connection = genome.genes.connect.get_mut(connection_in, connection_out);
                if connection.is_some(){
                    connection.unwrap().weight = value;
                }
            },
            Mutation::DeleteConnection(_, _) => {},
            Mutation::AddNode(_, _) => {},
            Mutation::DeleteNode(_) => {},
            Mutation::AddConnection(_, _, _)=> {},
            Mutation::NodeBiasChange(id, value) => {
                let node = genome.genes.nodes.get_mut(id);
                if node.is_some(){
                    node.unwrap().bias = value;
                }
            },
            Mutation::CppnInputMultiplierChange(_, _) => {}
        }
    }
}