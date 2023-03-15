use serde::{Deserialize, Serialize};
use crate::{neat::trainer::{configuration::Configuration, run_context::RunContext}, node_kind::NodeKind, activation_functions::ActivationFunction, common::{random::Random, NeatFloat, event_stream::{event::{EventType, Event}, event_recorder::EventRecorder}}};

use super::mutation::Mutation;

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct NodeGene {
    pub number: i32,
    pub kind: NodeKind,
    pub bias: NeatFloat,
    pub input_multiplier: NeatFloat,
    pub activation_function: ActivationFunction
}
impl NodeGene{
    pub fn new(index: i32, kind: NodeKind) -> NodeGene {
        let bias: NeatFloat = 1.0;
        if kind == NodeKind::Hidden{
            panic!("hidden nodes should be constructed using new_hidden()")
        }
        NodeGene{
            number: index,
            kind: kind,
            bias: bias,
            input_multiplier: 1.0,
            activation_function: ActivationFunction::empty()
        }
    }
    pub fn new_hidden(index: i32, activation_function: ActivationFunction) -> NodeGene {
        let bias: NeatFloat = 1.0;
        NodeGene{
            number: index,
            kind: NodeKind::Hidden,
            bias: bias,
            input_multiplier: Random::gen_range_f32(0.0 ,50.0),
            activation_function: activation_function
        }
    }
    pub fn mutate(&mut self, genome_id: &uuid::Uuid, configuration: &Configuration, run_context: &RunContext, mutations: &mut Vec<Mutation>){
        let random = Random::gen_range_f32(0.0 ,1.0);

        if random < configuration.mutation_node_bias_change_probability{
            let val: NeatFloat = Random::standard_normal() * configuration.mutation_node_bias_change_scale;
            
            if EventRecorder::has_subscription(configuration, EventType::MUTATION_NODE_CHANGE_BIAS){
                EventRecorder::record_event(configuration, &Event::mutation_node_bias_change(run_context, genome_id, self.number, val));
            }

            self.bias = (self.bias + val).clamp(configuration.node_bias_min_value, configuration.node_bias_max_value);

            mutations.push(Mutation::NodeBiasChange(self.number, self.bias));
        }

        if self.activation_function & ActivationFunction::for_cppn() == self.activation_function {
            
            if random < configuration.mutation_node_cppn_input_multiplier_change_probability{
                let val: NeatFloat = Random::standard_normal() * configuration.mutation_node_cppn_input_multiplier_change_scale;

                mutations.push(Mutation::CppnInputMultiplierChange(self.number, self.bias));

                self.input_multiplier += val; 
            }

            if random < configuration.mutation_node_cppn_input_multiplier_replace_probability{
                let val: NeatFloat = Random::standard_normal() * 20.0;
                mutations.push(Mutation::CppnInputMultiplierChange(self.number, self.bias));

                self.input_multiplier = val; 
            }
        } 
    }
    pub fn get_genetic_distance_from(&self, other: &NodeGene) -> NeatFloat{
        let mut result= NeatFloat::abs(self.bias - other.bias);
        if self.activation_function != other.activation_function{
            result += 1.0;
        }
        result
    }
}

#[test]
fn mutate_changes_bias() {
    let configuration  = Configuration::neat(Box::new(vec![
        NodeGene::new(1, NodeKind::Sensor),
        NodeGene::new(2, NodeKind::Output)
    ]), 0.0)
    .mutation_no_mutation()
    .mutation_node_bias_change_probability(1.0);

    let mut node = NodeGene::new_hidden(1, ActivationFunction::RELU);
    node.bias = 0.0;
    let id = uuid::Uuid::new_v4();
    node.mutate(&id, &configuration, &RunContext::new(2, 1), &mut Vec::new());
    assert_ne!(node.bias, 0.0);
}