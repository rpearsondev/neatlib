
use std::sync::Arc;
use neatlib::{neat::{
    genome::neat::{NeatGenome, mutation_mode::MutationMode }, 
    genome::genome::Genome,
    trainer::{run_context::RunContext, configuration::Configuration, node_conf::NodeConf}
}, renderer::renderer::{render_network_definition}};

fn main(){
    render_neat();
}

fn render_neat(){
    let node_conf = NodeConf::simple(4, 4);
    let mut run_context = RunContext::new(node_conf.len()+1, 0);
    let mut configuration  = Configuration::neat(node_conf
        , 0.0)
        .mutation_connection_allow_recurrent(false);

    let mut genome = Box::from(NeatGenome::minimal(&mut configuration, &mut run_context));
    for _ in 1..10000 {
            genome.mutate(&configuration, &mut run_context, MutationMode::Steady);
    }
    genome.genes.cleanup_orphan_nodes();
    render_network_definition(Arc::from(*genome));
}