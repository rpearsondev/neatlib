use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum SubstrateSetConnectionMode {
    Forward,
    ForwardOne
}