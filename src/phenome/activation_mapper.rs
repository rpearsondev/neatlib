use crate::{activation_functions::ActivationFunction as GeneActivationFunction, node_kind::NodeKind};
use crate::phenome::activations;
use super::ActivationFunction;

pub struct ActivationMapper;

impl ActivationMapper{
    pub fn map(node_kind: NodeKind, node_gene_activation: GeneActivationFunction) -> ActivationFunction {
        if node_kind == NodeKind::Output {
            return activations::linear;
        }else{
            if node_gene_activation == GeneActivationFunction::SIGMOID {
                return activations::sigmoid;
            }
            if node_gene_activation == GeneActivationFunction::RELU {
                return activations::relu;
            }
            if node_gene_activation == GeneActivationFunction::TANH {
                return activations::tanh;
            }
            if node_gene_activation == GeneActivationFunction::BINARY {
                return activations::binary;
            }
            if node_gene_activation == GeneActivationFunction::LINEAR_CLIP {
                return activations::linear_clip;
            }
            if node_gene_activation == GeneActivationFunction::LEAKY_RELU {
                return activations::leaky_relu;
            }
            if node_gene_activation == GeneActivationFunction::SINE {
                return activations::sin;
            }
            if node_gene_activation == GeneActivationFunction::BIPOLAR_SIGMOID {
                return activations::bipolar_sigmoid;
            }
            if node_gene_activation == GeneActivationFunction::GAUSSIAN {
                return activations::gaussian;
            }
            if node_gene_activation == GeneActivationFunction::BAND {
                return activations::band;
            }
            if node_gene_activation == GeneActivationFunction::BINARY_SIN {
                return activations::binary_sin;
            }
            if node_gene_activation == GeneActivationFunction::BINARY_GAUSSIAN {
                return activations::binary_gaussian;
            }
            if node_gene_activation == GeneActivationFunction::LINEAR_CLIP_GAUSSIAN {
                return activations::linear_clip_gaussian;
            }
            if node_gene_activation == GeneActivationFunction::INVERT {
                return activations::invert;
            }
        }
        activations::not_mapped
    }
}