use std::env;
use neatlib::{neat::{trainer::{node_conf::NodeConf, configuration::{Configuration}, fitness::{fitness_resolver::FitnessResolver, fitness_setter::FitnessSetter}, neat_trainer_host::neat_trainer_host::NeatTrainerHost, neat_trainer::NeatTrainer, activation_strategies::activation_strategies::ActivationStrategies, run_context::RunContext}, genome::{neat::{NeatGenome, mutation_mode::MutationMode}, genome::Genome}}, activation_functions::ActivationFunction, common::{NeatFloat}, phenome::Phenome, renderer::renderer::{NullSimulationRenderer, self}};
use image::{DynamicImage, GenericImage, Rgba};

//cargo run --release
//cargo run --release -- generate
const SIZE: i32 = 100;
pub fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "generate"{
        generate();
        return;
    }

    let image_path = "c:\\neatlib\\woop - small.jpg";
    let image_one = image::open(image_path).expect("Could not find test-image");
    
    let configuration = Configuration::neat(NodeConf::simple(4, 3), 0.0)
    .target_species(16)
    .mutation_node_available_activation_functions(ActivationFunction::for_cppn())
    .genome_minimal_genes_to_connect_ratio(0.5)
    .speciation_offspring_mode(neatlib::neat::trainer::configuration::OffSpringMode::AdjustedMemberRange)
    .speciation_drop_species_no_improvement_generations(20)
    .speciation_preserve_elite(true)
    .speciation_cross_species_reproduction_scale(0.05)
    .print_summary_interval(None)
    .print_summary_number_of_species_to_show(0)
    .population_size(300);

    let (host, client) = NeatTrainerHost::new(NeatTrainer::new(configuration), move  |trainer| {
        let current_gen = trainer.get_current_generation() ;
        if current_gen % 100 == 0{
            let best =  trainer.get_best_member_so_far();
            if best.is_some(){
                write_image( &Phenome::from_network_schema(&best.as_ref().unwrap().genome), current_gen, 1000);
            }
        }
        
        let mut strategy = ActivationStrategies::get_cpu_parallel(trainer);
        let mut fitness_setter = FitnessSetter::new();
        strategy.new_generation();
        
        let color = image_one.clone();
        let calc_fitness = move |phenotype: &Phenome, fitness_resolver: &mut FitnessResolver|{
            let primary = color.clone();
            calculate_fitness(phenotype, fitness_resolver, &primary);
        };
     
        strategy.compute(calc_fitness, &mut fitness_setter);
        fitness_setter.commit(trainer);
    });

    renderer::gui_runner(host, client, NullSimulationRenderer);
}

fn calculate_fitness(phenotype: &Phenome, fitness_resolver: &mut FitnessResolver, primary_image_color: &DynamicImage ) {
    let height: i32 = SIZE;
    let width: i32 = SIZE;
    let mut dynamic_image = DynamicImage::new_rgb8(width as u32, height as u32);
    let mut total_red: f32 = 0.0;
    let mut total_green: f32 = 0.0;
    let mut total_blue: f32 = 0.0;
    for x in 0..width as i32{
        for y in 0..height as i32{
            let x_f = (x as NeatFloat / width as NeatFloat) - 0.5;
            let y_f = (y as NeatFloat / height as NeatFloat) - 0.5;
            let x_f_s = x_f.abs().powf(2.0);
            let y_f_s = y_f.abs().powf(2.0);
            let dist = (x_f_s + y_f_s).sqrt();
            let max_f_s = f32::max(x_f, y_f);

            let result = phenotype.activate(
                &vec![
                    x_f,
                    y_f,
                    dist,
                    max_f_s
                ]);

                let results = [
                    (result[0] * 255.0) as u8, 
                    (result[1] * 255.0) as u8, 
                    (result[2] * 255.0) as u8, 
                    255
                    ];
            total_red += results[0] as f32 / 255.0;
            total_green += results[1] as f32 / 255.0;
            total_blue += results[1]as f32 / 255.0;
            
            let p: Rgba<u8> = image::Rgba(results);
            dynamic_image.put_pixel(x as u32, y as u32, p)
        }
    }

    let color_result = image_compare::rgb_hybrid_compare(&primary_image_color.to_rgb8(), &dynamic_image.to_rgb8()).expect("Images had different dimensions");
    let gray_result = image_compare::gray_similarity_structure(&image_compare::Algorithm::RootMeanSquared, &dynamic_image.to_luma8(), &dynamic_image.to_luma8()).expect("Images had different dimensions");
    
    let mut sum_vertical_middle = 0.0;
    let mut max_rgb_vertical = [0.0;3];
    let mut min_rgb_vertical = [0.0;3];
        for y in 0..height as i32{
        sum_vertical_middle+= gray_result.image.get_pixel((SIZE / 2) as u32, y as u32).0[0];

        let color_pixel = color_result.image.get_pixel((SIZE / 2) as u32, y as u32).0;
        if color_pixel[0] > max_rgb_vertical[0]{
            max_rgb_vertical[0] = color_pixel[0]
        }
        if color_pixel[0] < min_rgb_vertical[0]{
            min_rgb_vertical[0] = color_pixel[0]
        }

        if color_pixel[1] > max_rgb_vertical[1]{
            max_rgb_vertical[1] = color_pixel[1]
        }
        if color_pixel[1] < min_rgb_vertical[1]{
            min_rgb_vertical[1] = color_pixel[1]
        }

        if color_pixel[2] > max_rgb_vertical[2]{
            max_rgb_vertical[2] = color_pixel[2]
        }
        if color_pixel[2] < min_rgb_vertical[1]{
            min_rgb_vertical[2] = color_pixel[2]
        }
    }

    let mut sum_horizontal_middle = 0.0;
    for x in 0..width as i32{
        sum_horizontal_middle+= gray_result.image.get_pixel(x as u32, (SIZE / 3) as u32).0[0]
    }

    fitness_resolver.add_objective_fitness_component_with_novelty(1, 1.0, 50.0, color_result.score as NeatFloat,1);
    fitness_resolver.add_objective_fitness_component_with_novelty(2, 1.0, 50.0, gray_result.score as NeatFloat,1);
    fitness_resolver.add_novelty_component(3, total_red as f32 / (height * width) as f32, 5);
    fitness_resolver.add_novelty_component(4, total_green as f32 / (height * width) as f32, 5);
    fitness_resolver.add_novelty_component(5, total_blue as f32 / (height * width) as f32, 5);
    fitness_resolver.add_novelty_component(6, (total_red as f32 / (height * width) as f32) + (total_green as f32 / (height * width) as f32) + (total_blue as f32 / (height * width) as f32), 10);
    fitness_resolver.add_novelty_component(7, sum_vertical_middle as NeatFloat, 1);
    fitness_resolver.add_novelty_component(8, sum_horizontal_middle as NeatFloat, 1);
    fitness_resolver.add_novelty_component(9, max_rgb_vertical[0], 10);
    fitness_resolver.add_novelty_component(10, max_rgb_vertical[1], 10);
    fitness_resolver.add_novelty_component(11, max_rgb_vertical[2], 10);
    fitness_resolver.add_novelty_component(12, min_rgb_vertical[0], 10);
    fitness_resolver.add_novelty_component(13, min_rgb_vertical[1], 10);
    fitness_resolver.add_novelty_component(14, min_rgb_vertical[2], 10);
}

fn generate(){
    let configuration = Configuration::neat(NodeConf::simple(4, 3), 0.0)
    .target_species(8)
    .mutation_node_available_activation_functions(ActivationFunction::for_cppn())
    .genome_minimal_genes_to_connect_ratio(0.5)
    .speciation_offspring_mode(neatlib::neat::trainer::configuration::OffSpringMode::AdjustedMemberRange)
    .speciation_drop_species_no_improvement_generations(20)
    .speciation_preserve_elite(true)
    .speciation_cross_species_reproduction_scale(0.3)
    .print_summary_interval(None)
    .print_summary_number_of_species_to_show(0)
    .population_size(100);

    let mut run_context = RunContext::new(configuration.node_genes.len(), 2);

    for i in 0..1000{
        let mut genome = NeatGenome::minimal(&configuration, &mut run_context);
        
        for _ in 0..i*2{
            genome.mutate(&configuration, &mut run_context, MutationMode::Steady)
        }
        
        let phenome = Phenome::from_network_schema(&genome);

        write_image(&phenome, i, 500);
    }
}

fn write_image(phenotype: &Phenome, generation: u32, size: i32){
    let height: i32 = size;
    let width: i32 = size;
    let mut dynamic_image = DynamicImage::new_rgb8(width as u32, height as u32);
    for x in 0..width as i32{
        for y in 0..height as i32{

            let x_f = (x as NeatFloat / width as NeatFloat) - 0.5;
            let y_f = (y as NeatFloat / height as NeatFloat) - 0.5;
            let x_f_s = x_f.abs().powf(2.0);
            let y_f_s = y_f.abs().powf(2.0);
            let dist = (x_f_s + y_f_s).sqrt();
            let max_f_s = f32::max(x_f, y_f);

            let result = phenotype.activate(
                &vec![
                    x_f,
                    y_f,
                    dist,
                    max_f_s
                ]
            );

            let results = [
                (result[0] * 255.0) as u8, 
                (result[1] * 255.0) as u8, 
                (result[2] * 255.0) as u8, 
                255
            ];

            let p: Rgba<u8> = image::Rgba(results);
            dynamic_image.put_pixel(x as u32, y as u32, p)
        }
    }
    let result = dynamic_image.save(format!("c:\\neatlib\\cppn_generated\\cppn_image{}.jpg", generation));
    result.unwrap();
}