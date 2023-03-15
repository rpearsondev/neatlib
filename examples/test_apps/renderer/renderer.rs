
use std::sync::Arc;
use neatlib::{neat::{
    genome::neat::{NeatGenome, mutation_mode::MutationMode }, 
    genome::genome::Genome,
    trainer::{run_context::RunContext, configuration::Configuration, node_conf::NodeConf}
}, renderer::renderer::{render_network_definition, render_substrate_set}, hyperneat::substrate::{substrate_set::SubstrateSet, substrate_coordinate_scheme::SubstrateCoordinateScheme, substrate_geometric_organization::SubstrateGeometricOrganization, substrate::Substrate, substrate_type, substrate_set_connection_mode::SubstrateSetConnectionMode, substrate_set_cppn_mode::SubstrateSetCPPNMode}, activation_functions::ActivationFunction};

fn main(){
    if false{
        render_neat();
    }else{
        render_substrate();
    }
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

fn render_substrate(){
    let substrate_set = SubstrateSet::new(
        SubstrateCoordinateScheme::CenterOut, 
        SubstrateGeometricOrganization::Sandwich, 
        ActivationFunction::BIPOLAR_SIGMOID,
        SubstrateSetConnectionMode::Forward, 
        SubstrateSetCPPNMode::XyzAngleDistanceToXyzAngleDistance,
        Substrate::new(substrate_type::SubstrateType::Input, 28), 
        vec![
            Substrate::new(substrate_type::SubstrateType::Hidden, 50), 
            Substrate::new(substrate_type::SubstrateType::Hidden, 50)
        ],
        Substrate::new(substrate_type::SubstrateType::Output, 20));
        render_substrate_set(substrate_set);
}