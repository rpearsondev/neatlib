use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub enum SubstrateCoordinateScheme {
    CenterOut,
}