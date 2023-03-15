
use serde::{Serialize, Deserialize};
use crate::common::event_stream::event::Event;
use crate::common::{NeatFloat, event_stream::event::EventType};
use crate::common::random::Random;
use crate::neat::trainer::configuration::Configuration;
use crate::neat::trainer::run_context::RunContext;
use super::connect_genes::ConnectGenes;
use super::mutation::Mutation;
use super::mutation_add_mode::MutationNodeAddMode;
use super::mutation_mode::MutationMode;
use crate::node_kind::NodeKind;
use super::connect_gene::ConnectGene;
use super::node_gene::NodeGene;
use super::node_genes::NodeGenes;
use crate::common::event_stream::event_recorder::{EventRecorder};
#[derive(Serialize, Deserialize)]
#[derive(Clone, Default, Debug)]
pub struct NeatGenes {
    pub nodes: NodeGenes,
    pub connect: ConnectGenes,
}
impl NeatGenes{
    pub fn new(node_genes: NodeGenes, connect_genes: ConnectGenes) -> Self{
        Self { nodes: node_genes, connect: connect_genes }
    }
    pub fn mutate(&mut self, genome_id: &uuid::Uuid, configuration: &Configuration, run_context: &mut RunContext, mutations: &mut Vec<Mutation>, mutation_mode: MutationMode){
        let mut random = Random::gen_range_f32(0.0 ,1.0);

        let number_of_structural_mutations = match mutation_mode{
            MutationMode::Steady => 1,
            MutationMode::Optimistic => Random::gen_range_usize(1 , 3),
            MutationMode::FoolHardy =>  Random::gen_range_usize(1 , 6),
            MutationMode::ShootForTheMoon =>  Random::gen_range_usize(1 , 12),
        };
        
        for _ in 0..number_of_structural_mutations{
            if random <= configuration.mutation_node_add_probability {
                self.mutate_add_node(configuration, run_context, genome_id, mutations);
            }
        
            random = Random::gen_range_f32(0.0 ,1.0);
            if random <= configuration.mutation_node_delete_probability {
                self.mutate_delete_node(configuration, run_context, genome_id, mutations);
            }

            random = Random::gen_range_f32(0.0 ,1.0);
            if random <= configuration.mutation_connection_add_probability {
                let mut success = self.mutate_add_connection(genome_id, configuration, run_context, mutations);
                let mut c = 1;
                while !success && c <= 10 {
                    success = self.mutate_add_connection(genome_id, configuration, run_context, mutations);
                    c += 1;
                }
            }

            random = Random::gen_range_f32(0.0 ,1.0);
            if random <= configuration.mutation_connection_delete_probability {
                self.mutate_delete_connection(genome_id, configuration, run_context, mutations);
            }    
        } 

        for node in self.nodes.iter_mut() {
            node.mutate(genome_id, configuration, run_context, mutations);
        }

        for connection in self.connect.iter_mut() {
            if connection.is_enabled{
                connection.mutate(genome_id, configuration, run_context, mutations);
            }
        }
    }
    fn mutate_add_node(&mut self, configuration: &Configuration, run_context: &mut RunContext, genome_id: &uuid::Uuid, mutations: &mut Vec<Mutation>){
        let connection_to_split_option = self.connect.get_random_connect_gene();
        if connection_to_split_option.is_none() {
            return;
        }
        let connection_to_split = connection_to_split_option.unwrap();
        let new_node = run_context.node_index.get_hidden(configuration.mutation_node_available_activation_functions.get_random());
        
        //new
        let connection_in = run_context.gene_table.add(ConnectGene::new_with_weight(connection_to_split.connection_in, new_node.number,1.0 ,true));
        
        let connection_out: _;
        match configuration.mutation_node_add_mode{
            MutationNodeAddMode::DontDeleteExisting => {
                connection_out = run_context.gene_table.add(ConnectGene::new_with_weight(new_node.number, connection_to_split.connection_out,connection_to_split.weight / 2.0, true));
            } 
            MutationNodeAddMode::DeleteExisting => {
                connection_out = run_context.gene_table.add(ConnectGene::new_with_weight(new_node.number, connection_to_split.connection_out,connection_to_split.weight, true));
                self.connect.delete(connection_to_split.connection_in, connection_to_split.connection_out);
            } 
        }


        if EventRecorder::has_subscription(configuration, EventType::MUTATION_NODE_ADD){
            EventRecorder::record_event(configuration, &Event::mutation_node_add(run_context, genome_id, new_node.number, new_node.activation_function, connection_in.connection_in, connection_out.connection_out));
        }

        mutations.push(Mutation::AddNode(new_node.number, new_node.bias));

        self.connect.add(connection_in);
        self.connect.add(connection_out);
        self.nodes.add(new_node);

    }
    fn mutate_delete_node(&mut self, configuration: &Configuration, run_context: &mut RunContext, genome_id: &uuid::Uuid, mutations: &mut Vec<Mutation>){
        
        let node_to_delete_number = &self.nodes.get_random_hidden_node_number();
        match node_to_delete_number {
            Some(node_number) => {
                let n = *node_number;
                self.nodes.delete(n);
                self.connect.delete_connections_that_connect_to_node(n);
                
                if EventRecorder::has_subscription(configuration, EventType::MUTATION_NODE_DELETE){
                    EventRecorder::record_event(configuration, &&Event::mutation_node_delete(run_context, genome_id, n));
                }

                mutations.push(Mutation::DeleteNode(n))
            },
            None => {
                return;
            }
        }
        if configuration.mutation_remove_unconnected_nodes {
            self.cleanup_orphan_nodes();
        }
    }
    fn mutate_add_connection(&mut self, genome_id: &uuid::Uuid, configuration: &Configuration, run_context: &mut RunContext, mutations: &mut Vec<Mutation>) -> bool{
        
        let node_in_number_option = self.nodes.get_random_node_number(|n| n.kind != NodeKind::Output );
        match node_in_number_option {
            Some(node_in_number) => {
                let node_out_number_option = self.nodes.get_random_node_number(|n| n.number != node_in_number );
                if node_out_number_option == None{
                    return false;
                }

                //don't create duplicate connection
                let node_out_number = node_out_number_option.unwrap();
                
                if self.connect.get(node_in_number, node_out_number).is_some() {
                    return false;
                }

                //don't create connections between outputs
                let n_in = self.nodes.get(node_in_number);
                let n_out = self.nodes.get(node_out_number);
                let in_and_out = [n_in, n_out];
                if in_and_out.iter().all(|n| n.kind == NodeKind::Output){
                    return false;
                }

                //don't create connections between sensors
                if in_and_out.iter().all(|n| n.kind == NodeKind::Sensor){
                    return false;
                }

                // dont allow connection back to sensors
                if n_out.kind == NodeKind::Sensor {
                    return false;
                }

                //don't create recurrent or same layer connections if not allowed
                let mut is_recurrent = false;
                if !configuration.mutation_connection_allow_recurrent && self.node_has_input_connections(node_out_number){
                    let source_layer = self.get_distance_from_sensor(n_in, 0);
                    let target_layer = self.get_distance_from_sensor(n_out, 0);
                    if source_layer >= target_layer{
                        return false;
                    }
                }else{
                    let source_layer = self.get_distance_from_sensor(n_in, 0);
                    let target_layer = self.get_distance_from_sensor(n_out, 0);
                    is_recurrent = source_layer >= target_layer
                }

                let mut connection_to_add = run_context.gene_table.add(ConnectGene::new(node_in_number, node_out_number_option.unwrap(), true));
                connection_to_add.is_recurrent = is_recurrent;

                if Self::does_connection_cause_loop(self, &connection_to_add)
                {
                    return false;
                }

                if EventRecorder::has_subscription(configuration, EventType::MUTATION_CONNECTION_ADD){
                    EventRecorder::record_event(configuration, &&Event::mutation_connection_add(run_context, genome_id, connection_to_add.connection_in, connection_to_add.connection_out));
                }

                mutations.push(Mutation::AddConnection(connection_to_add.connection_in, connection_to_add.connection_out, connection_to_add.weight));

                let _ = &self.connect.add(connection_to_add);
              
                true
            },
            None => {
                return false;
            }
        }
    }
    fn mutate_delete_connection(&mut self, genome_id: &uuid::Uuid, configuration: &Configuration, run_context: &RunContext, mutations: &mut Vec<Mutation>){
        
        let connection_to_delete_option = self.connect.get_random_connect_gene();
        if connection_to_delete_option.is_some(){
            let connection_to_delete = connection_to_delete_option.unwrap();
            if EventRecorder::has_subscription(configuration, EventType::MUTATION_CONNECTION_DELETE){
                EventRecorder::record_event(configuration, &Event::mutation_connection_delete(run_context, genome_id, connection_to_delete.connection_in, connection_to_delete.connection_out));
            }

            mutations.push(Mutation::DeleteConnection(connection_to_delete.connection_in, connection_to_delete.connection_out));

            self.connect.delete(connection_to_delete.connection_in, connection_to_delete.connection_out)
        }
        if configuration.mutation_remove_unconnected_nodes {
            self.cleanup_orphan_nodes();
        }
    }
    pub fn does_connection_cause_loop(&self, new_connection: &ConnectGene) -> bool{
        self.get_previous_node(new_connection, new_connection.connection_out)
    }
    pub fn get_distance_from_sensor(&self, node: &NodeGene, hops: u32) -> u32{
        let connect_genes = self.connect.iter().filter(|n| n.connection_out == node.number).collect::<Vec<&ConnectGene>>();

        if connect_genes.len() == 0 {
            return hops;
        }

        let mut max:u32 = 0;
        for &connect_gene in connect_genes.iter(){
            let previous_node = self.nodes.get_opt( connect_gene.connection_in);
            
            let distance = match previous_node {
                Some(n) => {
                    self.get_distance_from_sensor(n, 1 + hops)
                } ,
                None => hops,
            };
            if distance > max{
                max = distance;
            }
        }

        return max;
    }
    fn get_previous_node(&self, new_connection: &ConnectGene, false_if_connection_out: i32) -> bool {
        let connection_in =  new_connection.connection_in;

        let feeders = self.connect.iter().filter(|c| c.connection_out == connection_in).collect::<Vec<&ConnectGene>>();
       
        for feeder in feeders.iter() {
            if feeder.connection_in == false_if_connection_out {
                return true;
            }
            if Self::get_previous_node(self ,feeder, false_if_connection_out) {
                return true;
            }
        }
        false
    }
    pub fn get_nodes_in_layer(&self, layer: u32) -> Vec<NodeGene> {
        let r = self.nodes.iter().filter(|n| {
            let distance = self.get_distance_from_sensor(n,0);
            return distance == layer;
        }).map(|n| n.clone() ).collect::<Vec<NodeGene>>();
        r
    }
    fn node_has_input_connections(&self, index: i32) -> bool {
        self.connect.iter().any(|c| c.connection_out == index)
    }
    pub fn cleanup_orphan_nodes(&mut self) {
        let node_genes = &mut self.nodes; 
        let connect_genes = &mut self.connect; 

        let get_orphaned_node_number= |connect_genes_clone: &Vec<ConnectGene>, node_genes_clone: NodeGenes| {
            for surviving_node in node_genes_clone.iter().filter(|n| n.kind == NodeKind::Hidden){
                let index =  surviving_node.number;
                let node_has_inputs = connect_genes_clone.iter().any(|c| c.is_enabled && c.connection_out == index);
                let node_has_outputs = connect_genes_clone.iter().any(|c| c.is_enabled && c.connection_in == index);
                if !node_has_inputs || !node_has_outputs {
                    return Some(index)
                }
            }
            return None;
        };

       let mut orphaned_node = get_orphaned_node_number(connect_genes.to_vec(), node_genes.clone());
       while orphaned_node != None {
           let orphaned_node_unwrapped  = orphaned_node.unwrap();
           connect_genes.delete_connections_that_connect_to_node(orphaned_node_unwrapped);
           node_genes.delete(orphaned_node_unwrapped);
           orphaned_node = get_orphaned_node_number(connect_genes.to_vec(), node_genes.clone());
       }
    }
    pub fn get_genetic_difference_distance_from(&mut self, other_genome: &NeatGenes, stop_when_hit: NeatFloat) -> NeatFloat{
        let max_node_count = NeatFloat::max(self.nodes.len() as NeatFloat, other_genome.nodes.len() as NeatFloat);
        if (self.nodes.len().abs_diff(other_genome.nodes.len()) as NeatFloat / NeatFloat::max(max_node_count, 1.0)) > stop_when_hit{
            return stop_when_hit as NeatFloat + 1.0;
        }

        let mut disjoint_nodes: NeatFloat = 0.0;
        let mut node_distance_sum: NeatFloat = 0.0;
        for self_node in self.nodes.iter(){
            let other_node = other_genome.nodes.get_opt(self_node.number);
            if other_node.is_some() {
                node_distance_sum += self_node.get_genetic_distance_from(other_node.unwrap())
            } else {
                disjoint_nodes +=1.0;
            }
        }
        for node_number in other_genome.nodes.get_node_numbers(){
            if !self.nodes.has_node(&node_number) {
                disjoint_nodes +=1.0;
            }
        }

        let node_distance = (node_distance_sum + disjoint_nodes) / NeatFloat::max(max_node_count, 1.0);

        if node_distance > stop_when_hit {
            return node_distance;
        }

        let max_connection_count = NeatFloat::max(self.connect.len() as NeatFloat, other_genome.connect.len() as NeatFloat);
        if (self.connect.len().abs_diff(other_genome.connect.len()) as NeatFloat / NeatFloat::max(max_connection_count, 1.0)) > stop_when_hit{
            return stop_when_hit as NeatFloat + 1.0;
        }

        let mut connection_distance_sum:NeatFloat = 0.0;
        let mut disjoint_connections:NeatFloat = 0.0;
        for connection in self.connect.iter(){
            let other_connection = other_genome.connect.get_by_hash(connection.connection_hash);
            if other_connection.is_some() {
                connection_distance_sum += connection.get_genetic_distance_from(other_connection.unwrap())
            } else {
                disjoint_connections +=1.0;
            }
        }
        for other_connection in other_genome.connect.iter(){
            if !self.connect.contains_by_hash(other_connection.connection_hash) {
                disjoint_connections +=1.0;
            }
        }
        
        let connection_distance = (connection_distance_sum + disjoint_connections) / NeatFloat::max(max_connection_count, 1.0);
        let result = node_distance + connection_distance;
        result
    }
    pub fn get_complexity(&self) -> NeatFloat{
        self.connect.len() as NeatFloat
    }
}