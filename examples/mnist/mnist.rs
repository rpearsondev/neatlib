use neatlib::{neat::trainer::{node_conf::NodeConf, configuration::{Configuration, OffSpringMode}, neat_trainer::NeatTrainer, activation_strategies::activation_strategies::ActivationStrategies, fitness::{fitness_setter::{FitnessSetter}, fitness_resolver::FitnessResolver}, run_context::RunContext, neat_trainer_host::neat_trainer_host::NeatTrainerHost}, activation_functions::ActivationFunction, common::NeatFloat, renderer::renderer::{self, NullSimulationRenderer}};

pub fn main(){

    let gpu = NeuralModelGpuRunner::new();
    
    let (training_data_source, testing_data_source) = DataLoader::from_directory("c:\\temp", "(?<class>.*).jpg");
    
    let model = NeuralModel::new();
    pipeline.add_layer(InputLayer::new(vec![28, 28, 1]));
    pipeline.add_layer(Conv::new(ConvOptions{}));
    pipeline.add_layer(MapPool::new(MaxPoolOptions{}));
    pipeline.add_layer(Dense::new(vec![4], DenseOptions{}));
    pipeline.add_layer(OutputLayer::new(vec![9]));

    let get_fitness = | phenotype: &Phenome, fitness_resolver: &mut FitnessResolver| {
            for mut parameter in model.get_parameters_mut(){
                parameter.set_value(phenotype.activate(parameter.get_dimensions()));
            }

            let evaluation = model.evaluate(training_data_source);
           
            fitness_resolver.add_objective_fitness_component(0, 1.0, 0.0, evaluation.loss);

            let mut output_component_id = 1;
            for output in evaluation.outputs{
                fitness_resolver.add_novelty_component(output_component_id, output, 50);
                output_component_id+=1;
            }
    };

    let configuration = Configuration::neat(NodeConf::simple(1, 1), success_threshold)
    .target_species(20)
    .mutation_node_available_activation_functions(ActivationFunction::TANH| ActivationFunction::SIGMOID | ActivationFunction::RELU | ActivationFunction::BINARY)
    .genome_minimal_genes_to_connect_ratio(0.0)
    .run_name("mnist".to_string())
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