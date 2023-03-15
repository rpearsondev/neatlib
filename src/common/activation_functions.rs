use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use super::random::Random;

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct ActivationFunction : u32 {
        const SIGMOID               = 0b00000000000000000000000000000001;
        const RELU                  = 0b00000000000000000000000000000010;
        const TANH                  = 0b00000000000000000000000000000100;
        const BINARY                = 0b00000000000000000000000000001000;
        const LINEAR_CLIP           = 0b00000000000000000000000000010000;
        const LEAKY_RELU            = 0b00000000000000000000000000100000;
        const SINE                  = 0b00000000000000000000000001000000;
        const BIPOLAR_SIGMOID       = 0b00000000000000000000000010000000;
        const GAUSSIAN              = 0b00000000000000000000000100000000;
        const BAND               = 0b00000000000000000000001000000000;
        const BINARY_SIN            = 0b00000000000000000000010000000000;
        const BINARY_GAUSSIAN       = 0b00000000000000000000100000000000;
        const LINEAR_CLIP_GAUSSIAN  = 0b00000000000000000001000000000000;
        const INVERT                = 0b00000000000000000010000000000000;
    }
}
impl ActivationFunction{
    pub fn get_all() -> [ActivationFunction; 14]{
        [
            ActivationFunction::SIGMOID, 
            ActivationFunction::RELU, 
            ActivationFunction::TANH, 
            ActivationFunction::BINARY, 
            ActivationFunction::LINEAR_CLIP, 
            ActivationFunction::LEAKY_RELU,
            ActivationFunction::SINE,
            ActivationFunction::BIPOLAR_SIGMOID,
            ActivationFunction::GAUSSIAN,
            ActivationFunction::BAND,
            ActivationFunction::BINARY_SIN,
            ActivationFunction::BINARY_GAUSSIAN,
            ActivationFunction::LINEAR_CLIP_GAUSSIAN,
            ActivationFunction::INVERT,
        ]
      }
      pub fn for_cppn() -> ActivationFunction{
            ActivationFunction::LINEAR_CLIP| 
            ActivationFunction::SINE|
            ActivationFunction::BIPOLAR_SIGMOID|
            ActivationFunction::GAUSSIAN|
            ActivationFunction::BAND|
            ActivationFunction::BINARY|
            ActivationFunction::BINARY_SIN|
            ActivationFunction::BINARY_GAUSSIAN|
            ActivationFunction::LINEAR_CLIP_GAUSSIAN|
            ActivationFunction::INVERT
      }
    pub fn get_random(&self) -> Self{
        let mut available: Vec<ActivationFunction> = vec![];

        for activation in ActivationFunction::get_all(){
            if self.intersects(activation) {
                available.push(activation);
            }
        }
        let random = Random::gen_range_usize(0 ,available.len());
        available[random]
    }
}
impl Default for ActivationFunction {
    fn default() -> ActivationFunction {
        ActivationFunction::SIGMOID | ActivationFunction::RELU
    }
}

#[test]
fn get_random_works_one_million_times() {
    let a : ActivationFunction = ActivationFunction::SIGMOID | ActivationFunction::RELU | ActivationFunction::TANH | ActivationFunction::BINARY;
    for _ in 0..1000_000{
        let result = a.get_random();
        let is_valid = result == ActivationFunction::SIGMOID || result == ActivationFunction::RELU || result == ActivationFunction::TANH || result == ActivationFunction::BINARY;
        assert!(is_valid);
    }
}