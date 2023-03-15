use serde::{Serialize, Deserialize};
use crate::{neat::genome::neat::{node_gene::{NodeGene}}, activation_functions::ActivationFunction};

#[derive(Debug, Clone, Default)]
#[derive(Serialize, Deserialize)]
pub struct NoneGeneIndex{
    index: usize,
    initial_count: usize
}

impl NoneGeneIndex{
    pub fn new(initial_count: usize) -> Self{
        NoneGeneIndex {
            initial_count: initial_count,
            index: initial_count + 1
        }
    }
    pub fn get_hidden(&mut self, activation_function: ActivationFunction) -> NodeGene {
            self.index += 1;
            return NodeGene::new_hidden(
                self.index as i32,
                activation_function
            )
    }
    pub fn clear(&mut self){
        self.index = self.initial_count + 1;
    }
}