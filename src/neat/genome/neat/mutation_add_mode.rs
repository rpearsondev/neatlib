use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub enum MutationNodeAddMode{
    DontDeleteExisting,
    DeleteExisting,
}