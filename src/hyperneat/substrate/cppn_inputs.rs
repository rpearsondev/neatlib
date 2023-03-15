use serde::{Serialize, Deserialize};
use super::{substrate_set_cppn_mode::SubstrateSetCPPNMode, substrate_node::SubstrateNode};

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(Serialize, Deserialize)]
pub struct CppnInputs{
    pub inputs: Vec<CppnInput>,
    pub cppn_mode: SubstrateSetCPPNMode,
    pub expected_number_of_cppn_outputs: u32,
    pub expected_number_of_cppn_inputs: u32
}

#[derive(Debug, Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct CppnInput{
    pub in_node: SubstrateNode,
    pub out_node: SubstrateNode
}
