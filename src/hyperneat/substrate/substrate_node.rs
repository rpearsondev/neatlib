use serde::{Serialize, Deserialize};

use crate::{activation_functions::ActivationFunction, common::NeatFloat};

use super::substrate_type::SubstrateType;

#[derive(Debug, Clone, PartialEq, Copy)]
#[derive(Serialize, Deserialize)]
pub struct SubstrateNode{
    pub node_id: i32,
    pub node_type: SubstrateType,
    pub activation: ActivationFunction,
    pub cube_coordinates: SubstrateCubeCoordinates,
    pub substrate_coordinates: SubstrateSubstrateCoordinates,
}

#[derive(Debug, Clone, PartialEq, Copy)]
#[derive(Serialize, Deserialize)]
pub struct SubstrateCubeCoordinates{
    pub x: NeatFloat,
    pub y: NeatFloat,
    pub z: NeatFloat,
    pub angle_from_center: NeatFloat,
    pub distance_from_center: NeatFloat
}

#[derive(Debug, Clone, PartialEq, Copy)]
#[derive(Serialize, Deserialize)]
pub struct SubstrateSubstrateCoordinates{
    pub x: i32,
    pub y: i32,
    pub z: i32
}

