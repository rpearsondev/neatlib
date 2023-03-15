
#[cfg(test)]
use crate::neat::genome::genome::Genome;
#[cfg(test)]
use super::{node_gene::{NodeGene}, connect_gene::ConnectGene, neat_genome::NeatGenome};
use crate::{neat::{trainer::{run_context::RunContext, configuration::Configuration}, genome::neat::{mutation_mode::MutationMode, mutation_add_mode::MutationNodeAddMode}}, node_kind::NodeKind, activation_functions::ActivationFunction, common::{network_definition_node_layer_resolver::NetworkDefinitionNodeLayerResolver, NeatFloat}};

#[test]
fn minimal_add_connect_genes() {
    let configuration  = Configuration::neat(
        Box::new(vec![
            NodeGene::new(1, NodeKind::Sensor),
            NodeGene::new(2, NodeKind::Output)
        ]), 0.0);

    let minimal_genome = NeatGenome::minimal(&configuration, &mut RunContext::new(2, 0));

    assert_eq!(minimal_genome.genes.connect.len(), 1);
    assert!(minimal_genome.genes.connect.get(1, 2).is_some());
}

#[test]
fn mutate_add_node_adds_node(){
    let configuration  = Configuration::neat(
        Box::new(vec![
            NodeGene::new(1, NodeKind::Sensor),
            NodeGene::new(2, NodeKind::Output)
        ]), 0.0)
        .mutation_no_mutation()
        .mutation_node_add_probability(1.0)
        .mutation_node_add_mode(MutationNodeAddMode::DeleteExisting);

    let mut run_context =  RunContext::new(2, 0);
    let mut minimal_genome = NeatGenome::minimal(&configuration, &mut run_context);
    minimal_genome.mutate(&configuration, &mut run_context, MutationMode::Steady);

    assert_eq!(minimal_genome.genes.connect.len(), 2);
    assert_eq!(minimal_genome.genes.nodes.len(), 3);
}

#[test]
fn mutate_twice_add_node_adds_node(){
    let configuration  = Configuration::neat(
        Box::new(vec![
            NodeGene::new(1, NodeKind::Sensor),
            NodeGene::new(2, NodeKind::Output)
        ]), 0.0)
        .mutation_no_mutation()
        .mutation_node_add_probability(1.0)
        .mutation_node_add_mode(MutationNodeAddMode::DeleteExisting);

    let mut run_context =  RunContext::new(2, 0);
    let mut minimal_genome = NeatGenome::minimal(&configuration, &mut run_context);
    minimal_genome.mutate(&configuration, &mut run_context, MutationMode::Steady);
    minimal_genome.mutate(&configuration, &mut run_context, MutationMode::Steady);

    assert_eq!(minimal_genome.genes.connect.len(), 3);
    assert_eq!(minimal_genome.genes.nodes.len(), 4);
}

#[test]
fn mutate_delete_node_deletes_nodes() {
    let mut configuration  = Configuration::neat(
        Box::new(vec![
            NodeGene::new(1, NodeKind::Sensor),
            NodeGene::new(2, NodeKind::Output)
        ]), 0.0)
        .mutation_no_mutation()
        .mutation_node_add_probability(1.0)
        .mutation_node_add_mode(MutationNodeAddMode::DeleteExisting);

    let mut run_context =  RunContext::new(2, 0);
    let mut minimal_genome = NeatGenome::minimal(&configuration, &mut run_context);
    minimal_genome.mutate(&configuration, &mut run_context, MutationMode::Steady);
    minimal_genome.mutate(&configuration, &mut run_context, MutationMode::Steady);
    assert_eq!(minimal_genome.genes.connect.len(), 3);
    assert_eq!(minimal_genome.genes.nodes.len(), 4);

    configuration = configuration
    .mutation_no_mutation()
    .mutation_node_add_probability(0.0)
    .mutation_node_delete_probability(1.0)
    .mutation_node_add_mode(MutationNodeAddMode::DeleteExisting);

    minimal_genome.mutate(&configuration, &mut run_context, MutationMode::Steady);
    minimal_genome.mutate(&configuration, &mut run_context, MutationMode::Steady);

    assert_eq!(minimal_genome.genes.connect.len(), 0);
    assert_eq!(minimal_genome.genes.nodes.len(), 2);
}

#[test]
fn mutate_add_connection_add_connection(){
    let configuration  = Configuration::neat(
        Box::new(vec![
            NodeGene::new(1, NodeKind::Sensor),
            NodeGene::new(2, NodeKind::Output)
        ]), 0.0)
        .mutation_no_mutation()
        .mutation_connection_add_probability(1.0);

    let mut run_context =  RunContext::new(2, 0);
    let mut minimal_genome = NeatGenome::minimal(&configuration, &mut run_context);
    minimal_genome.genes.connect.delete(1,2);
    minimal_genome.mutate(&configuration, &mut run_context, MutationMode::Steady);
    assert_eq!(minimal_genome.genes.connect.len(), 1);
}

#[test]
fn mutate_add_connection_add_connection_does_not_add_duplicates(){
    let configuration  = Configuration::neat(
        Box::new(vec![
            NodeGene::new(1, NodeKind::Sensor),
            NodeGene::new(2, NodeKind::Output)
        ]), 0.0)
        .mutation_no_mutation()
        .mutation_connection_add_probability(1.0);

    let mut run_context =  RunContext::new(2, 0);
    let mut minimal_genome = NeatGenome::minimal(&configuration, &mut run_context);
    minimal_genome.genes.connect.clear();
    minimal_genome.mutate(&configuration, &mut run_context, MutationMode::Steady);
    minimal_genome.mutate(&configuration, &mut run_context, MutationMode::Steady);
    assert_eq!(minimal_genome.genes.connect.len(), 1);
}

#[test]
fn mutate_add_connection_does_not_create_connection_between_outputs(){
    let configuration  = Configuration::neat(
        Box::new(vec![
            NodeGene::new(1, NodeKind::Sensor), // will be manually removed
            NodeGene::new(2, NodeKind::Output),
            NodeGene::new(3, NodeKind::Output)
        ]), 0.0)
        .mutation_no_mutation()
        .mutation_connection_add_probability(1.0);

    let mut run_context =  RunContext::new(3, 0);
    let mut minimal_genome = NeatGenome::minimal(&configuration, &mut run_context);

    minimal_genome.genes.nodes.delete(1);
    minimal_genome.genes.connect.clear();

    minimal_genome.mutate(&configuration, &mut run_context, MutationMode::Steady);
    minimal_genome.mutate(&configuration, &mut run_context, MutationMode::Steady);
    assert_eq!(minimal_genome.genes.connect.len(), 0);
}

#[test]
fn mutate_add_connection_does_not_create_connection_between_sensors(){
    let configuration  = Configuration::neat(
        Box::new(vec![
            NodeGene::new(1, NodeKind::Sensor),
            NodeGene::new(2, NodeKind::Sensor),
            NodeGene::new(3, NodeKind::Output)
        ]), 0.0)
        .mutation_no_mutation()
        .mutation_connection_add_probability(1.0);
    
    let mut run_context =  RunContext::new(3, 0);
    let mut minimal_genome = NeatGenome::minimal(&configuration, &mut run_context);

    minimal_genome.genes.nodes.delete(3);
    minimal_genome.genes.connect.clear();

    minimal_genome.mutate(&configuration, &mut run_context, MutationMode::Steady);
    minimal_genome.mutate(&configuration, &mut run_context, MutationMode::Steady);
    assert_eq!(minimal_genome.genes.connect.len(), 0);
}

#[test]
fn get_nodes_in_layer() {
    let configuration  = Configuration::neat(
    Box::new(vec![
        NodeGene::new(1, NodeKind::Sensor),
        NodeGene::new_hidden(2, ActivationFunction::RELU),
        NodeGene::new_hidden(3, ActivationFunction::RELU),
        NodeGene::new(4, NodeKind::Output)
    ]), 0.0);

    let mut run_context =  RunContext::new(4, 0);
    let mut minimal_genome = NeatGenome::minimal(&configuration, &mut run_context);

    minimal_genome.genes.connect.clear();
    minimal_genome.genes.connect.add(ConnectGene::new(1, 2, true));
    minimal_genome.genes.connect.add(ConnectGene::new(1, 3, true));
    minimal_genome.genes.connect.add(ConnectGene::new(2, 4, true));
    minimal_genome.genes.connect.add(ConnectGene::new(3, 4, true));

    let layer_zero_nodes = minimal_genome.genes.get_nodes_in_layer(0);
    let layer_one_nodes = minimal_genome.genes.get_nodes_in_layer(1);
    let layer_two_nodes = minimal_genome.genes.get_nodes_in_layer(2);

    assert_eq!(layer_zero_nodes[0].number, 1);
    assert_eq!(layer_one_nodes[0].number, 2);
    assert_eq!(layer_one_nodes[1].number, 3);
    assert_eq!(layer_two_nodes[0].number, 4);
}

#[test]
fn get_genetic_difference_distance_zero_for_identical_genomes(){
        let configuration  = Configuration::neat(
        Box::new(vec![
            NodeGene::new(1, NodeKind::Sensor),
            NodeGene::new_hidden(2, ActivationFunction::RELU),
            NodeGene::new(3, NodeKind::Output)
        ]), 0.0);

    let mut run_context =  RunContext::new(3, 0);
    let mut minimal_genome = NeatGenome::minimal(&configuration, &mut run_context);

    minimal_genome.genes.connect.clear();
    minimal_genome.genes.connect.add(ConnectGene::new_with_weight(1, 2, 1.0, true));
    minimal_genome.genes.connect.add(ConnectGene::new_with_weight(2, 3, 1.0, true));
    minimal_genome.genes.connect.add(ConnectGene::new_with_weight(2, 3, 1.0, true));

    let clone = minimal_genome.clone();
    let genetic_distance = minimal_genome.get_genetic_difference_distance_from(&clone, NeatFloat::MAX);

    assert_eq!(genetic_distance, 0.0);
}

#[test]
fn get_genetic_difference_distance_1_for_disjoint_node(){
    let configuration  = Configuration::neat(
    Box::new(vec![
        NodeGene::new(1, NodeKind::Sensor),
        NodeGene::new_hidden(2, ActivationFunction::RELU),
        NodeGene::new(3, NodeKind::Output)
    ]), 0.0)
    .mutation_node_available_activation_functions(ActivationFunction::RELU)
    .mutation_no_mutation()
    .mutation_node_add_probability(1.0)
    .mutation_node_add_mode(MutationNodeAddMode::DeleteExisting);

    let mut run_context =  RunContext::new(3, 0);
    let mut left = NeatGenome::minimal(&configuration, &mut run_context);
    left.genes.connect.clear();
    left.genes.connect.add(ConnectGene::new_with_weight(1, 2, 1.0 ,true));
    left.genes.connect.add(ConnectGene::new_with_weight(2, 3, 1.0, true));

    let mut right = left.clone();
    right.mutate(&configuration, &mut run_context, MutationMode::Steady);

    let genetic_distance = left.get_genetic_difference_distance_from(&right, NeatFloat::MAX);

    //1.5? .. (1 / 4 == 0.25) for the nodes + (3 / 3) because the mutation added a connection added 1 connection, but 3 are disjoint in total.
    assert_eq!(genetic_distance, 1.25);
}

#[test]
fn get_genetic_difference_distance_correct_for_node_with_different_activation(){
    let configuration  = Configuration::neat(
    Box::new(vec![
        NodeGene::new(1, NodeKind::Sensor),
        NodeGene::new_hidden(2, ActivationFunction::RELU),
        NodeGene::new(3, NodeKind::Output)
    ]), 0.0);

    let mut run_context =  RunContext::new(3, 0);
    let mut left = NeatGenome::minimal(&configuration, &mut run_context);
    left.genes.connect.clear();
    left.genes.connect.add(ConnectGene::new_with_weight(1, 2, 1.0 ,true));
    left.genes.connect.add(ConnectGene::new_with_weight(2, 2, 1.0, true));
    left.genes.connect.add(ConnectGene::new_with_weight(2, 3, 1.0, true));

    let mut right = left.clone();
    right.genes.nodes.get_mut_unchecked(2).activation_function = ActivationFunction::BINARY;

    let genetic_distance = left.get_genetic_difference_distance_from(&right, NeatFloat::MAX);

    assert_eq!(genetic_distance, 1.0 / 3.0);
}

#[test]
fn get_genetic_difference_distance_correct_for_node_with_different_bias(){
    let configuration  = Configuration::neat(
    Box::new(vec![
        NodeGene::new(1, NodeKind::Sensor),
        NodeGene::new_hidden(2, ActivationFunction::RELU),
        NodeGene::new(3, NodeKind::Output)
    ]), 0.0);

    let mut run_context =  RunContext::new(3, 0);
    let mut left = NeatGenome::minimal(&configuration, &mut run_context);
    left.genes.connect.clear();
    left.genes.connect.add(ConnectGene::new_with_weight(1, 2, 1.0 ,true));
    left.genes.connect.add(ConnectGene::new_with_weight(2, 2, 1.0, true));
    left.genes.connect.add(ConnectGene::new_with_weight(2, 3, 1.0, true));

    let mut right = left.clone();
    right.genes.nodes.get_mut_unchecked(2).bias = 0.0;

    let genetic_distance = left.get_genetic_difference_distance_from(&right, NeatFloat::MAX);

    assert_eq!(genetic_distance, 1.0 / 3.0);
}

#[test]
fn get_genetic_difference_distance_correct_for_connections_with_different_weights(){
    let configuration  = Configuration::neat(
    Box::new(vec![
        NodeGene::new(1, NodeKind::Sensor),
        NodeGene::new_hidden(2, ActivationFunction::RELU),
        NodeGene::new(3, NodeKind::Output)
    ]), 0.0);

    let mut run_context =  RunContext::new(3, 0);
    let mut left = NeatGenome::minimal(&configuration, &mut run_context);
    left.genes.connect.clear();
    left.genes.connect.add(ConnectGene::new_with_weight(1, 2, 1.0 ,true));
    left.genes.connect.add(ConnectGene::new_with_weight(2, 2, 1.0, true));
    left.genes.connect.add(ConnectGene::new_with_weight(2, 3, 1.0, true));

    let mut right = left.clone();
    right.genes.connect.get_mut_unchecked(2, 2).weight = -1.0;

    let genetic_distance = left.get_genetic_difference_distance_from(&right, NeatFloat::MAX);

    assert_eq!(genetic_distance, 2.0 / 3.0);
}

#[test]
fn get_node_layers(){
    let configuration  = Configuration::neat(
    Box::new(vec![
        NodeGene::new(1, NodeKind::Sensor),
        NodeGene::new_hidden(2, ActivationFunction::RELU),
        NodeGene::new_hidden(3, ActivationFunction::RELU),
        NodeGene::new_hidden(4, ActivationFunction::RELU),

        NodeGene::new_hidden(5, ActivationFunction::RELU),
        NodeGene::new_hidden(6, ActivationFunction::RELU),
        NodeGene::new_hidden(7, ActivationFunction::RELU),

        NodeGene::new(8, NodeKind::Output)
    ]), 0.0);

    let mut run_context =  RunContext::new(8, 0);
    let mut g = NeatGenome::minimal(&configuration, &mut run_context);

    g.genes.connect.clear();

    g.genes.connect.add(ConnectGene::new_with_weight(1, 2, 1.0 ,true));
    g.genes.connect.add(ConnectGene::new_with_weight(2, 3, 1.0, true));
    g.genes.connect.add(ConnectGene::new_with_weight(2, 3, 1.0, true));
    g.genes.connect.add(ConnectGene::new_with_weight(3, 4, 1.0, true));
    g.genes.connect.add(ConnectGene::new_with_weight(4, 8, 1.0, true));

    g.genes.connect.add(ConnectGene::new_with_weight(1, 5, 1.0, true));
    g.genes.connect.add(ConnectGene::new_with_weight(5, 6, 1.0, true));
    g.genes.connect.add(ConnectGene::new_with_weight(6, 7, 1.0, true));
    g.genes.connect.add(ConnectGene::new_with_weight(7, 8, 1.0, true));

    let node_layers = NetworkDefinitionNodeLayerResolver::get_node_layers(&g, false);
   
    println!("{:?}", node_layers.layers);
}

#[test]
fn get_node_layers_recurrent(){
    let configuration  = Configuration::neat(
    Box::new(vec![
        NodeGene::new(1, NodeKind::Sensor),
        NodeGene::new_hidden(2, ActivationFunction::RELU),
        NodeGene::new_hidden(3, ActivationFunction::RELU),
        NodeGene::new(4, NodeKind::Output)
    ]), 0.0);

    let mut run_context =  RunContext::new(8, 0);
    let mut g = NeatGenome::minimal(&configuration, &mut run_context);

    g.genes.connect.clear();

    g.genes.connect.add(ConnectGene::new_with_weight(1, 2, 1.0 ,true));
    g.genes.connect.add(ConnectGene::new_with_weight(2, 4, 1.0, true));
    g.genes.connect.add(ConnectGene::new_with_weight(2, 3, 1.0, true));
    let mut recurrent2 = ConnectGene::new_with_weight(3, 1, 1.0, true);
    recurrent2.is_recurrent = true;
    g.genes.connect.add(recurrent2);
    g.genes.connect.add(ConnectGene::new_with_weight(2, 4, 1.0, true));

    //render_network_definition(g.clone());
    let resolved_node_layers = NetworkDefinitionNodeLayerResolver::get_node_layers(&g, false);
    println!("{:?}", resolved_node_layers.layers.iter().map(|l| l.iter().map(|n| n.identity).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>());
}

#[test]
fn get_node_layers_debug(){
    let configuration  = Configuration::neat(
    Box::new(vec![
        NodeGene::new(1, NodeKind::Sensor),
        NodeGene::new_hidden(2, ActivationFunction::RELU),
        NodeGene::new(3, NodeKind::Output)
    ]), 0.0);

    let mut run_context =  RunContext::new(3, 0);
    let mut g = NeatGenome::minimal(&configuration, &mut run_context);
    g.genes.connect.clear();
    g.genes.connect.add(ConnectGene::new_with_weight(1, 2, 1.0 ,true));
    g.genes.connect.add(ConnectGene::new_with_weight(2, 3, 1.0, true));

    let resolved_node_layers = NetworkDefinitionNodeLayerResolver::get_node_layers(&g, false);
    assert_eq!(resolved_node_layers.layers[0][0].identity, 1);
    assert_eq!(resolved_node_layers.layers[1][0].identity, 2);
    assert_eq!(resolved_node_layers.layers[2][0].identity, 3);
    
    println!("{:?}", resolved_node_layers.layers);
}