use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum NodeKind {
    Sensor,
    Output,
    Hidden
}
