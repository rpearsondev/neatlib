use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use wasm_stopwatch::Stopwatch;
use crate::{activation_functions::{ActivationFunction as GeneActivationFunction, self}, neat::{genome::{neat::{node_gene::{NodeGene}, NeatGenome, connect_gene::ConnectGene, mutation_mode::MutationMode}, genome::Genome}, trainer::{run_context::RunContext, configuration::Configuration, node_conf::NodeConf}}, node_kind::NodeKind, common::NeatFloat, phenome::Phenome};

use super::CpuPhenome;
#[test]
fn maps_all_activations() {
    for activation in GeneActivationFunction::get_all(){
        let mut configuration  = Configuration::neat(
            Box::new(vec![
                NodeGene::new(1, NodeKind::Sensor),
                NodeGene::new_hidden(2, activation),
                NodeGene::new(3, NodeKind::Output)
            ]), 0.0);
    
        let mut run_context = RunContext::new(3, 0);
        let mut minimal_genome = NeatGenome::minimal(&mut configuration, &mut run_context);
        minimal_genome.genes.connect.clear();
        minimal_genome.genes.connect.add(ConnectGene::new_with_weight(1, 2, 1.0, true));
        minimal_genome.genes.connect.add(ConnectGene::new_with_weight(2, 3, 1.0, true));
        minimal_genome.genes.nodes.get_mut_unchecked(2).bias = 0.0;
        minimal_genome.genes.nodes.get_mut_unchecked(3).bias = 0.0;

        let phenotype = CpuPhenome::from_network_schema(&minimal_genome);

        println!("genotype activation: {:?}", minimal_genome.genes.nodes.get_mut_unchecked(1).activation_function);
        println!("phenotype activation: {:?}", phenotype.layers[1].nodes[0].activation);
        
        //activation will panic if not set
        phenotype.activate(&vec![1.0]);
    }
}

#[test]
fn activate_minimal_one_million_times() {
    let mut configuration  = Configuration::neat(
        Box::new(vec![
            NodeGene::new(1, NodeKind::Sensor),
            NodeGene::new(2, NodeKind::Output)
        ]), 0.0)
        .mutation_node_available_activation_functions(GeneActivationFunction::RELU);

    let mut run_context = RunContext::new(2, 0);
    let mut minimal_genome = NeatGenome::minimal(&mut configuration, &mut run_context);
    minimal_genome.genes.connect.get_mut_unchecked(1, 2).weight = 0.5;
    minimal_genome.genes.nodes.get_mut_unchecked(2).bias = 0.0;

    let phenotype = CpuPhenome::from_network_schema(&minimal_genome);

    for _i in 1..1000_000 {
        let result = phenotype.activate(&vec![2.0]);
        assert_eq!(result[0], 1.0);
    }
}

#[test]
fn activations_mapping_works_correctly_at_scale() {
    let mut configuration = Configuration::neat(NodeConf::simple(2, 1), 0.0)
    .mutation_connection_add_probability(1.0)
    .mutation_node_add_probability(1.0);
    let mut run_context = RunContext::new(3, 0);
    
    for _i in 1..1000 {
        let mut minimal_genome = NeatGenome::minimal(&mut configuration, &mut run_context);

        for _ in 0..10{
            minimal_genome.mutate(&mut configuration, &mut run_context, MutationMode::Steady);
        }
        
        let phenotype = CpuPhenome::from_network_schema(&minimal_genome);
        phenotype.activate(&vec![2.0]);
    }
}

#[test]
fn activate_two_inputs_one_million_times() {
    let mut configuration  = Configuration::neat(
        Box::new(vec![
            NodeGene::new(1, NodeKind::Sensor),
            NodeGene::new(2, NodeKind::Sensor),
            NodeGene::new(3, NodeKind::Output)
        ]), 0.0)
        .mutation_node_available_activation_functions(GeneActivationFunction::RELU);

    let mut run_context = RunContext::new(3, 0);
    let mut minimal_genome = NeatGenome::minimal(&mut configuration, &mut run_context);
    minimal_genome.genes.connect.get_mut_unchecked(1, 3).weight = 0.2;
    minimal_genome.genes.connect.get_mut_unchecked(2, 3).weight = 0.3;
    minimal_genome.genes.nodes.get_mut_unchecked(2).bias = 0.0;
    minimal_genome.genes.nodes.get_mut_unchecked(3).bias = 0.0;

    let phenotype = CpuPhenome::from_network_schema(&minimal_genome);

    for _i in 1..1000_000{
        let result = phenotype.activate(&vec![3.0, 3.0]);
        assert_eq!(result[0], 1.5);
    }
}

#[test]
fn activate_recurrent_connection() {
    let mut configuration  = Configuration::neat(
        Box::new(vec![
            NodeGene::new(1, NodeKind::Sensor),
            NodeGene::new_hidden(2, activation_functions::ActivationFunction::RELU),
            NodeGene::new(3, NodeKind::Output)
        ]), 0.0)
        .mutation_node_available_activation_functions(GeneActivationFunction::RELU);

    let mut run_context = RunContext::new(3, 0);
    let mut minimal_genome = NeatGenome::minimal(&mut configuration, &mut run_context);
    minimal_genome.genes.connect.clear();

    minimal_genome.genes.connect.add(ConnectGene::new_with_weight(1, 2, 0.5, true));
    minimal_genome.genes.connect.add(ConnectGene::new_with_weight(2, 3, 0.5, true));
    let mut recurrent_connection = ConnectGene::new_with_weight(2, 1, 0.4, true);
    recurrent_connection.is_recurrent = true;
    minimal_genome.genes.connect.add(recurrent_connection);

    let phenotype = CpuPhenome::from_network_schema(&minimal_genome);

    let node_results: Vec<f64> = Vec::new();
    let result1 = phenotype.activate_recurrent(&vec![vec![3.0], vec![3.0]]);
    println!("{:?}", node_results);
    let result2 = phenotype.activate(&vec![3.0]);
    let result3 = phenotype.activate(&vec![3.0]);

    println!("r1: {:?}  r2: {:?}  r3:{:?}", result1, result2, result3);
    assert_ne!(result1, result2);
}


#[test]
fn activate_recurrent_connection_output_to_hidden() {
    let mut configuration  = Configuration::neat(
        Box::new(vec![
            NodeGene::new(1, NodeKind::Sensor),
            NodeGene::new_hidden(2, activation_functions::ActivationFunction::RELU),
            NodeGene::new(3, NodeKind::Output)
        ]), 0.0)
        .mutation_node_available_activation_functions(GeneActivationFunction::RELU);

    let mut run_context = RunContext::new(3, 0);
    let mut minimal_genome = NeatGenome::minimal(&mut configuration, &mut run_context);

    minimal_genome.genes.connect.add(ConnectGene::new(3, 2, true));

    let phenotype = CpuPhenome::from_network_schema(&minimal_genome);

    let result1 = phenotype.activate_recurrent(&vec![vec![3.0], vec![3.0]]);
    let result2 = phenotype.activate(&vec![3.0]);

    println!("r1: {:?}  r2: {:?}", result1, result2);
    assert_ne!(result1, result2);
    for _i in 1..1000_000{
        phenotype.activate(&vec![3.0]);
    }
}


#[test]
fn activate_one_inputs_with_one_hidden_one_million_times() {
    let mut configuration  = Configuration::neat(
        Box::new(vec![
            NodeGene::new(1, NodeKind::Sensor),
            NodeGene::new_hidden(2, GeneActivationFunction::RELU),
            NodeGene::new(3, NodeKind::Output)
        ]), 0.0);

    let mut run_context = RunContext::new(3, 0);
    let mut minimal_genome = NeatGenome::minimal(&mut configuration, &mut run_context);
    minimal_genome.genes.connect.clear();
    minimal_genome.genes.connect.add(ConnectGene::new_with_weight(1, 2, 0.8, true));
    minimal_genome.genes.connect.add(ConnectGene::new_with_weight(2, 3, 0.5, true));
    minimal_genome.genes.nodes.get_mut_unchecked(2).bias = 0.0;
    minimal_genome.genes.nodes.get_mut_unchecked(3).bias = 0.0;


    let phenome = CpuPhenome::from_network_schema(&minimal_genome);

    for _i in 1..1000_000{
        let result = phenome.activate(&vec![1.0]);
        assert_eq!(result[0], 0.4);
    } 
}

#[test]
fn activate_one_inputs_with_one_hidden_connection_skips_layer_one_million_times() {
    let mut configuration  = Configuration::neat(
        Box::new(vec![
            NodeGene::new(1, NodeKind::Sensor),
            NodeGene::new_hidden(2, GeneActivationFunction::RELU),
            NodeGene::new(3, NodeKind::Output)
        ]), 0.0);

    let mut run_context = RunContext::new(3, 0);
    let mut minimal_genome = NeatGenome::minimal(&mut configuration, &mut run_context);
    minimal_genome.genes.connect.clear();
    minimal_genome.genes.connect.add(ConnectGene::new_with_weight(1, 2, 1.0, true));
    minimal_genome.genes.connect.add(ConnectGene::new_with_weight(2, 3, 1.0, true));
    minimal_genome.genes.connect.add(ConnectGene::new_with_weight(1, 3, 0.8, true));
    minimal_genome.genes.nodes.get_mut_unchecked(2).bias = 0.0;
    minimal_genome.genes.nodes.get_mut_unchecked(3).bias = 0.0;


    let phenotype = CpuPhenome::from_network_schema(&minimal_genome);

    for _i in 1..1000_000{
        let result = phenotype.activate(&vec![1.0]);
        assert_eq!(result[0], 1.8);
    } 
}

#[test]
fn phenotype_result_indexing_works_layer_skips() {
    let mut configuration  = Configuration::neat(
        Box::new(vec![
            NodeGene::new(1, NodeKind::Sensor),
            NodeGene::new(2, NodeKind::Sensor),
            NodeGene::new(3, NodeKind::Output),
            NodeGene::new_hidden(4, GeneActivationFunction::RELU)
        ]), 0.0);

    let mut run_context = RunContext::new(4, 0);
    let mut minimal_genome = NeatGenome::minimal(&mut configuration, &mut run_context);
    minimal_genome.genes.connect.clear();
    minimal_genome.genes.connect.add(ConnectGene::new_with_weight(1, 3, 1.0, true));
    minimal_genome.genes.connect.add(ConnectGene::new_with_weight(2, 3, 1.0, true));
    minimal_genome.genes.connect.add(ConnectGene::new_with_weight(1, 4, 1.0, true));
    minimal_genome.genes.connect.add(ConnectGene::new_with_weight(4, 3, 1.0, true));
    minimal_genome.genes.nodes.get_mut_unchecked(1).bias = 0.0;
    minimal_genome.genes.nodes.get_mut_unchecked(2).bias = 0.0;
    minimal_genome.genes.nodes.get_mut_unchecked(3).bias = 0.0;
    minimal_genome.genes.nodes.get_mut_unchecked(4).bias = 0.0;

    let phenotype = CpuPhenome::from_network_schema(&minimal_genome);

    let result = phenotype.activate(&vec![1.0]);
    assert_eq!(result[0], 2.0);
}

#[test]
fn activate_with_complex_network_does_not_panic_one_million_activations_parallel_and_serial() {
    //to test in release: cargo test --release phenotype::activate_with_complex_network_does_not_panic_one_million_activations -- --nocapture  
    let mut configuration  = Configuration::neat(
    Box::new(vec![
        NodeGene::new(1, NodeKind::Sensor),
        NodeGene::new(2, NodeKind::Output)
    ]), 0.0)
    .mutation_connection_add_probability(0.18)
    .mutation_node_add_probability(0.18)
    .mutation_remove_unconnected_nodes(false);

    let mut run_context = RunContext::new(2, 0);
    let mut minimal_genome= NeatGenome::minimal(&mut configuration, &mut run_context);    

    let number_of_mutations = 300;
    let mut sw = Stopwatch::new();
    for _i in 1..number_of_mutations{
        minimal_genome.mutate(&mut configuration, &mut run_context, MutationMode::Steady);
    }
    println!("{} mutations took {}ms",number_of_mutations, sw.get_time());
    
    let mut clone = minimal_genome.clone();
    clone.genes.cleanup_orphan_nodes();
    println!("connect_genes:{:.2} node:genes:{:.2}, layers:{:.2}", clone.genes.connect.len(), clone.genes.nodes.len(), clone.genes.get_distance_from_sensor(&NodeGene::new(3, NodeKind::Output), 1));    

    let mut phenotype = CpuPhenome::from_network_schema(&minimal_genome);
    sw.reset();
    let range = 2.0;
    let steps  = 1000_000;
    let ids = (1..steps as i32).collect::<Vec<i32>>();
    let par_iter = ids.par_iter();
    par_iter.for_each(|i| {
        let v = ((range / steps as NeatFloat) * *i as NeatFloat) - (range / 2.0);
        phenotype.activate(&vec![v]);
    });
    println!("{} parallel activations took {}ms",steps, sw.get_time());


    phenotype = CpuPhenome::from_network_schema(&minimal_genome);
    sw.reset();
    for i in 1..steps {
        let v = ((range / steps as NeatFloat) * i as NeatFloat) - (range / 2.0);
        phenotype.activate(&vec![v]);
    };
    
    println!("{} serial activations took {}ms",steps, sw.get_time());
    
}
