use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(Serialize, Deserialize)]
pub enum SubstrateSetCPPNMode {
    #[default]
    XyzAngleDistanceToXyzAngleDistance
}