mod phenome;
pub mod node_index_lookup;
mod phenome_layer_node;
mod phenome_layer_node_connection;
mod phenome_layer;
mod activation_mapper;
mod activations;
pub use phenome::Phenome;
use crate::common::NeatFloat;

pub type ActivationFunction =  fn(NeatFloat, NeatFloat) -> NeatFloat;
#[cfg(test)]
mod tests;