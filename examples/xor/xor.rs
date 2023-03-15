

use neatlib::{neat::trainer::{configuration::{Configuration}, node_conf::NodeConf, neat_trainer::NeatTrainer, activation_strategies::activation_strategies::ActivationStrategies, fitness::{fitness_setter::{FitnessSetter}, fitness_resolver::{FitnessResolver}}, neat_trainer_host::{neat_trainer_host::NeatTrainerHost}}, activation_functions::ActivationFunction, phenome::Phenome, renderer::renderer::{self, NullSimulationRenderer}};
pub fn main(){
    let success_threshold = 3.99;
    
    let calculate_fitness = | phenotype: &Phenome, fitness_resolver: &mut FitnessResolver | {
        let xor_results= &[[0.0, 0.0, 0.0], [0.0, 1.0, 1.0], [1.0, 0.0, 1.0], [1.0, 1.0, 0.0]];

        let mut component_id = 0;
        for test in xor_results {
            let result = phenotype.activate(&vec![test[0], test[1]])[0];
            let expected_result = test[2];
            fitness_resolver.add_objective_fitness_component_with_novelty(component_id, 1.0, expected_result, result, 100);
            component_id += 1;
        }
    };

    let configuration = Configuration::neat(NodeConf::simple(2, 1), success_threshold)
    .target_species(20)
    .mutation_node_available_activation_functions(ActivationFunction::TANH)
    .genome_minimal_genes_to_connect_ratio(1.0)
    .speciation_drop_species_no_improvement_generations(20)
    .speciation_preserve_elite(true)
    .print_summary_interval(None)
    .print_summary_number_of_species_to_show(0)
    .population_size(1000);

    let (host, client) = NeatTrainerHost::new(NeatTrainer::new(configuration), move  |trainer| {
        let mut strategy = ActivationStrategies::get_cpu_parallel(trainer);
        let mut fitness_setter = FitnessSetter::new();
        strategy.new_generation();
        strategy.compute(calculate_fitness, &mut fitness_setter);
        fitness_setter.commit(trainer);
    });

    renderer::gui_runner(host, client, NullSimulationRenderer);
}