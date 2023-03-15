use crate::common::NeatFloat;

#[derive(Debug, Copy, Clone)]
pub struct PhenomeLayerNodeConnection{
    pub from_node_array_index: usize,
    pub weight: NeatFloat
}
