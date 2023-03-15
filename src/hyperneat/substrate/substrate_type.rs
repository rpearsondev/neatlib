use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Copy)]
#[derive(Serialize, Deserialize)]
pub enum SubstrateType {
    Input,
    Output,
    Hidden,
}