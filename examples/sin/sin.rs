use neatlib::{neat::{trainer::{node_conf::NodeConf, configuration::{Configuration}, neat_trainer::NeatTrainer, activation_strategies::activation_strategies::ActivationStrategies, fitness::{fitness_setter::{FitnessSetter}, fitness_resolver::FitnessResolver}, neat_trainer_host::neat_trainer_host::NeatTrainerHost}}, activation_functions::ActivationFunction, common::NeatFloat, renderer::renderer::{self, NullSimulationRenderer}, phenome::Phenome};

//cargo run --release --example sin
pub fn main(){
    let success_threshold = 18.9995;

    let get_fitness = | phenotype: &dyn Phenome, fitness_resolver: &mut FitnessResolver| {
        let mut sensor_inputs: Vec<NeatFloat> = Vec::with_capacity(20);
        let mut expected_results: Vec<NeatFloat> = Vec::with_capacity(20);
        // Create a non-linear curve to approximate.
        for i in 1..20 {
            let input = 2.0 / i as NeatFloat;
            let expected_result = (5.0 / i as NeatFloat).sin();
            sensor_inputs.push(input);
            expected_results.push(expected_result);
        }
    

        for i in 0..expected_results.len() { 
            let network_outputs = phenotype.activate(&vec![sensor_inputs[i]]);
            let expected_result = expected_results[i];
            let network_output = network_outputs[0];
            fitness_resolver.add_objective_fitness_component_with_novelty(i as u32, 1.0, expected_result, network_output, 100);
        }
    };

    let configuration = Configuration::neat(NodeConf::simple(1, 1), success_threshold)
    .target_species(20)
    .mutation_node_available_activation_functions(ActivationFunction::TANH | ActivationFunction::SIGMOID | ActivationFunction::RELU | ActivationFunction::BINARY)
    .genome_minimal_genes_to_connect_ratio(0.0)
    .run_name("sin".to_string())
    .print_summary_interval(None)
    .survival_threshold(0.1)
    .mutation_connection_weight_change_scale(0.01)
    .speciation_remove_stagnant_species_generations(500)
    .population_size(1000);
   
    let trainer = NeatTrainer::new(configuration);

    let (host, client) = NeatTrainerHost::new(trainer, move |trainer| {
        trainer.new_generation();
        let mut gpu_activation_strategy = ActivationStrategies::get_cpu_parallel(trainer);
        let mut fitness_setter = FitnessSetter::new();
        gpu_activation_strategy.compute(get_fitness, &mut fitness_setter);
        fitness_setter.commit(trainer);
    });

    renderer::gui_runner(host, client, NullSimulationRenderer);
}