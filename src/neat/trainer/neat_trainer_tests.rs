#![allow(unused)]
use std::borrow::{Borrow, BorrowMut};
use crate::{
    phenome::Phenome,
    neat::genome::genome::Genome, 
    neat::trainer::{configuration::{Configuration, OffSpringMode}, node_conf::NodeConf, fitness::{fitness_setter::{FitnessSetter}, Fitness}}, 
    neat::{genome::neat::{node_gene::NodeGene}, trainer::activation_strategies::activation_strategies::ActivationStrategies}, 
    neat::population::{ GenerationMember}, activation_functions::ActivationFunction, common::NeatFloat};
use super::{neat_trainer::NeatTrainer, fitness::fitness_resolver::FitnessResolver};
#[test]
#[ignore = "For performance only"]
fn xor_mutate_test_parallel_times_x(){
//cargo test --release neat_trainer_tests::xor_mutate_test_parallel_times_x -- --nocapture --ignored
    let success_threshold = 3.999;
    
    let calculate_fitness = | phenotype: &dyn Phenome, fitness_resolver: &mut FitnessResolver | {
        let xor_results= &[[0.0, 0.0, 0.0], [0.0, 1.0, 1.0], [1.0, 0.0, 1.0], [1.0, 1.0, 0.0]];

        let mut fitness_component = 0;
        for test in xor_results{
            let result = phenotype.activate(&vec![test[0], test[1]])[0];
            let expected_result = test[2];
            fitness_resolver.add_objective_fitness_component_with_novelty(fitness_component, 1.0, expected_result, result, 100);
            fitness_component +=1;
        }
    };
    let runs = 15;
    let mut number_of_gens =  0;
    for _ in 0..runs{
        let configuration = Configuration::neat(NodeConf::simple(2, 1), success_threshold)
        .target_species(5)
        .mutation_node_available_activation_functions(ActivationFunction::BINARY)
        .genome_minimal_genes_to_connect_ratio(1.0)
        .speciation_offspring_mode(OffSpringMode::AdjustedMemberRange)
        .speciation_drop_species_no_improvement_generations(20)
        .print_summary_interval(Some(100))
        .print_summary_number_of_species_to_show(0)
        .population_size(1000);

        let mut neat_trainer = NeatTrainer::new(configuration);
        while !neat_trainer.has_met_success() {
            let generation  = neat_trainer.new_generation();
            let mut activation_strategy = ActivationStrategies::get_cpu_parallel(&mut neat_trainer);
            let mut fitness_setter = FitnessSetter::new();
            activation_strategy.compute(calculate_fitness, &mut fitness_setter);
            fitness_setter.commit(&mut neat_trainer);
        }

        number_of_gens += neat_trainer.run_context.current_generation;
    }

    println!("avg gens:{}", number_of_gens / runs)
}

#[test]
#[ignore = "For performance only"]
fn xor_relu_mutate_test_parallel_times_x(){
//cargo test --release neat_trainer_tests::xor_relu_mutate_test_parallel_times_x -- --nocapture --ignored
    let success_threshold = 3.999;
    
    let calculate_fitness = | phenotype: &dyn Phenome, fitness_resolver: &mut FitnessResolver | {
        let xor_results= &[[0.0, 0.0, 0.0], [0.0, 1.0, 1.0], [1.0, 0.0, 1.0], [1.0, 1.0, 0.0]];

        let mut fitness_component = 0;
        for test in xor_results{
            let result = phenotype.activate(&vec![test[0], test[1]])[0];
            let expected_result = test[2];
            fitness_resolver.add_objective_fitness_component_with_novelty(fitness_component, 1.0, expected_result, result, 100);
            fitness_component +=1;
        }
    };
    let runs = 15;
    let mut number_of_gens =  0;
    for _ in 0..runs{
        let configuration = Configuration::neat(NodeConf::simple(2, 1), success_threshold)
        .target_species(5)
        .mutation_node_available_activation_functions(ActivationFunction::RELU)
        .genome_minimal_genes_to_connect_ratio(0.0)
        .speciation_offspring_mode(OffSpringMode::AdjustedMemberRange)
        .speciation_drop_species_no_improvement_generations(8)
        .print_summary_interval(Some(100))
        .print_summary_number_of_species_to_show(0)
        .population_size(1000);

        let mut neat_trainer = NeatTrainer::new(configuration);
        while !neat_trainer.has_met_success() {
            let generation  = neat_trainer.new_generation();
            let mut activation_strategy = ActivationStrategies::get_cpu_parallel(&mut neat_trainer);
            let mut fitness_setter = FitnessSetter::new();
            activation_strategy.compute(calculate_fitness, &mut fitness_setter);
            fitness_setter.commit(&mut neat_trainer);
        }

        number_of_gens += neat_trainer.run_context.current_generation;
    }

    println!("avg gens:{}", number_of_gens / runs)
}

#[test]
fn single_generation() {
    let configuration = Configuration::neat(NodeConf::simple(2, 1), 0.0)
        .population_size(1000)
        .speciation_genetic_distance_threshold(3.1)
        .mutation_connection_weight_change_probability(0.4)
        .mutation_connection_add_probability(0.3)
        .mutation_node_add_probability(0.5);
    
    let mut neat_trainer = NeatTrainer::new(configuration);

    neat_trainer.new_generation();
    
    let total_members = neat_trainer.run_context.species_index.iter().map(|(_, s)| s.members.len()).sum::<usize>();
    let all_members_there = total_members == neat_trainer.configuration.population_size as usize;
    assert_eq!(neat_trainer.run_context.current_generation, 1);
    assert!(all_members_there);
}

#[test]
fn speciation_test() {
    let configuration = Configuration::neat(NodeConf::simple(2, 1), 0.0)
        .population_size(10)
        .target_species(20)
        .speciation_genetic_distance_threshold(3.1)
        .mutation_connection_weight_change_probability(0.4)
        .mutation_connection_add_probability(0.3)
        .mutation_node_add_probability(0.5);
    
    let mut neat_trainer = NeatTrainer::new(configuration);
   
    let print_stats = |neat_trainer: &NeatTrainer| {

        let expected_number_of_pop = &(neat_trainer.run_context).species_index.iter().map(|(_, s)| s.members.len() as i32).sum::<i32>();
        let num_species = &(neat_trainer.run_context).species_index.len();
        println!("gen:{} pop: {} expected_pop: {} num_species: {}",
            neat_trainer.run_context.current_generation,
            neat_trainer.members.len(),
            expected_number_of_pop,
            num_species
        );

        for (s, species) in &((neat_trainer.run_context).species_index){
        
            println!("id: {}, offspring: {}, av fitness: {}, from gen {} , no.members {}",
                s, 
                species.allowed_number_of_offspring_based_on_objective_fitness, 
                species.objective_fitness.average, 
                species.created_generation,
                species.members.len(),
            );
        }
        
    };

    for _ in 0..10{
        neat_trainer.new_generation();
        print_stats(&neat_trainer);
  
        for member in neat_trainer.members.iter_mut(){
            member.genome.set_objective_fitness(1.1);
        }
    }
}