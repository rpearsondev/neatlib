use std::collections::{hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};
use crate::common::NeatFloat;
use crate::common::event_stream::event::{EventType, Event};
use crate::common::event_stream::event_recorder::EventRecorder;
use crate::common::random::Random;
#[allow(unused_imports)]
use crate::neat::genome::neat::node_gene::NodeGene;
#[allow(unused_imports)]
use crate::neat::trainer::configuration::Configuration;
use crate::neat::trainer::run_context::RunContext;
#[allow(unused_imports)]
use crate::node_kind::NodeKind;
use serde::{Serialize, Deserialize};

use super::mutation::Mutation;

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct ConnectGene {
    pub connection_in: i32,
    pub connection_out: i32,
    pub weight: NeatFloat,
    pub is_enabled: bool,
    pub is_recurrent: bool,
    pub connection_hash: u64
}

impl ConnectGene {
    pub fn new(connection_in: i32,connection_out: i32,is_enabled: bool)  -> Self {
        let mut hasher = DefaultHasher::new();
        connection_in.hash(&mut hasher);
        connection_out.hash(&mut hasher);
        let connection_hash = hasher.finish();

        let random = Random::gen_range_f32(-1.0 ,1.0);

        ConnectGene{
            connection_in,
            connection_out,
            weight: random,
            is_enabled,
            is_recurrent: false,
            connection_hash
        }
    }
    pub fn compute_hash(connection_in: i32,connection_out: i32) -> u64{
        
        let mut hasher = DefaultHasher::new();
        connection_in.hash(&mut hasher);
        connection_out.hash(&mut hasher);
        let connection_hash = hasher.finish();
        connection_hash
    }
    pub fn new_with_weight(connection_in: i32,connection_out: i32,weight: NeatFloat,is_enabled: bool)  -> Self {
        
        let connection_hash = Self::compute_hash(connection_in, connection_out);
        ConnectGene{
            connection_in,
            connection_out,
            weight,
            is_enabled,
            is_recurrent: false,
            connection_hash
        }
   }
    pub fn mutate(&mut self, genome_id: &uuid::Uuid, configuration: &Configuration, run_context: &RunContext, mutations: &mut Vec<Mutation>) {
        
        let mut random = Random::gen_range_f32(0.0 ,1.0);

        if random < configuration.mutation_connection_weight_change_probability{
            let val: NeatFloat = Random::standard_normal() * configuration.mutation_connection_weight_change_scale;

            let new_weight = (self.weight + val).clamp(configuration.connection_weight_min_value, configuration.connection_weight_max_value);
            if EventRecorder::has_subscription(configuration, EventType::MUTATION_CONNECTION_WEIGHT_CHANGE){
                EventRecorder::record_event(configuration, &Event::mutation_connection_weight_change(run_context, genome_id, self.connection_in, self.connection_out,new_weight, self.weight));
            }

            mutations.push(Mutation::ConnectionChangeWeight(self.connection_in, self.connection_out, new_weight));

            self.weight = new_weight
        }

        random = Random::gen_range_f32(0.0 ,1.0);
        if random < configuration.mutation_connection_weight_replace_probability {
            let val: NeatFloat = Random::standard_normal();

            let new_weight = val.clamp(configuration.connection_weight_min_value, configuration.connection_weight_max_value);
            if EventRecorder::has_subscription(configuration, EventType::MUTATION_CONNECTION_WEIGHT_REPLACE){
                EventRecorder::record_event(configuration, &Event::mutation_connection_weight_replace(run_context, genome_id, self.connection_in, self.connection_out,new_weight, self.weight));
            }

            mutations.push(Mutation::ConnectionChangeWeight(self.connection_in, self.connection_out, new_weight));

            self.weight = new_weight;
        }

        let should_disable = Random::gen_bool(configuration.mutation_connection_disable_probability);
        if should_disable{
            if EventRecorder::has_subscription(configuration, EventType::MUTATION_CONNECTION_DISABLED){
                EventRecorder::record_event(configuration, &Event::mutation_connection_disabled(run_context, genome_id, self.connection_in, self.connection_out));
            }
            self.is_enabled = false;
        }
    }
    pub fn get_genetic_distance_from(&self, other: &ConnectGene) -> NeatFloat {
        let mut result= NeatFloat::abs(self.weight - other.weight);
        if self.is_enabled != other.is_enabled{
            result += 1.0;
        }
        result
    }
}

#[test]
fn mutate_changes_weight() {
    let configuration  = Configuration::neat(Box::new(vec![
        NodeGene::new(1, NodeKind::Sensor),
        NodeGene::new(2, NodeKind::Output)
    ]),0.0)
    .mutation_no_mutation()
    .mutation_connection_weight_change_probability(1.0);

    let mut connection = ConnectGene::new_with_weight(1, 2, 0.0, true);
    connection.mutate(&uuid::Uuid::new_v4(), &configuration, &RunContext::new(2, 2), &mut Vec::new());
    assert_ne!(connection.weight, 0.0);
}

#[test]
fn mutate_replaces_weight() {
    let configuration  = Configuration::neat(Box::new(vec![
        NodeGene::new(1, NodeKind::Sensor),
        NodeGene::new(2, NodeKind::Output)
    ]), 0.0)
    .mutation_no_mutation()
    .mutation_connection_weight_replace_probability(1.0);

    let mut connection = ConnectGene::new_with_weight(1, 2, 0.0, true);
    connection.mutate(&uuid::Uuid::new_v4(), &configuration, &RunContext::new(2, 2),&mut Vec::new());
    assert_ne!(connection.weight, 0.0);
}