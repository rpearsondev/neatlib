use image::{DynamicImage, GenericImage, Rgba};
use image::GenericImageView;
use neatlib::activation_functions::ActivationFunction;
use neatlib::common::NeatFloat;
use neatlib::neat::genome::genome::Genome;
use neatlib::neat::genome::neat::NeatGenome;
use neatlib::neat::genome::neat::mutation_mode::MutationMode;
use neatlib::neat::trainer::configuration::{Configuration, self};
use neatlib::neat::trainer::node_conf::NodeConf;
use neatlib::neat::trainer::run_context::RunContext;
use neatlib::phenome::{Phenome};

//translating from: http://eplex.cs.ucf.edu/ESHyperNEAT/

//Create quadtree structure to define the nodes
// #Input nodes
// Foreach network_input -> 
// {
//         let quadTreeRoot = DivisisionAndInitialization (input.x, input.y, isInput=true)
//         let (connections) = PruneAndExtract(input.x, input.y, quadTreeRoot)
//         Foreach inputConnection in connections
//     {
//             if node_doesnt_already_exist() {
//                 add_node(inputConnection.x, inputConnection.y)
//             }
//     }
// }
// #Hidden nodes
// for i in 0..iteration_depth{
//     Foreach network_hidden_node -> 
//     {
//             let quadTreeRoot = DivisisionAndInitialization (network_hidden_node.x, network_hidden_node.y, isInput=true)
//             let (connections) = PruneAndExtract(network_hidden_node.x, network_hidden_node.y, quadTreeRoot)
//             Foreach hiddenConnection in connections
//         {
//                 if node_doesnt_already_exist() {
//                     add_node(hiddenConnection.x, hiddenConnection.y)
//                 }
//         }
//     }
// }
// #Output nodes
// for i in 0..iteration_depth{
//     Foreach network_hidden_node -> 
//     {
//             let quadTreeRoot = DivisisionAndInitialization (network_hidden_node.x, network_hidden_node.y, isInput=false)
//             let (outputConnections) = PruneAndExtract(network_hidden_node.x, network_hidden_node.y, quadTreeRoot)
       
//     }
// }

// let finalConnections = inputConnections.contact(hiddenConnections).concat(outputConnections)
// # this point you should have nodes and connections, then you need to clear any nodes that are not connected to input or output 
// turn it into network

pub fn main(){
    
    let img = image::open("assets/images/img1").unwrap();
    println!("width {}", img.width());

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
                (result[0] * 255.0) as u8,  //R = The connection weight
                (result[1] * 255.0) as u8,  //G = LEO (Link expression output) if more than 0 then generate connection
                (result[2] * 255.0) as u8,  // B
                255
            ];

            let p: Rgba<u8> = image::Rgba(results);
            dynamic_image.put_pixel(x as u32, y as u32, p)
        }
    }
    let result = dynamic_image.save(format!("c:\\neatlib\\cppn_generated\\cppn_image{}.jpg", generation));
    result.unwrap();
}