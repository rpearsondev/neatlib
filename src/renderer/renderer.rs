use std::sync::Arc;
use bevy::prelude::*;
use bevy_egui::{EguiPlugin};
use crate::neat::genome::neat::NeatGenome;
use crate::neat::population::GenerationMember;
use crate::neat::trainer::configuration::Configuration;
use crate::neat::trainer::generation_stats::GenerationStats;
use crate::neat::trainer::neat_trainer_host::from_host_events::FromHostEvent;
use crate::neat::trainer::neat_trainer_host::models::generic_operation::GenericOperation;
use crate::neat::trainer::neat_trainer_host::models::species::SpeciesList;
use crate::neat::trainer::neat_trainer_host::neat_trainer_host::{NeatTrainerHostClient, NeatTrainerHost};
use crate::neat::trainer::neat_trainer_host::to_host_events::ToHostEvents;
use crate::neat::trainer::config_regulators::config_regulator::ConfigRegulator;
use super::plugins::neat_settings_gui::NeatSettingsGui;
use super::plugins::network_renderer::NetworkDefinitionRenderer;

pub struct NeatTrainerState{
    pub run_until: Option<u32>,
    pub reset_requested: bool,
    pub current_generation: u32,
    pub configuration: Configuration,
    pub best_member_so_far: Option<GenerationMember<NeatGenome>>,
    pub species_list: SpeciesList,
    pub last_ten_thousand_generations_stats: Vec<GenerationStats>,
    pub config_regulators: Vec<ConfigRegulator>,
    pub generic_operations_queue: Vec<GenericOperation>
}

pub struct NullSimulationRenderer;
impl Plugin for NullSimulationRenderer {
    fn build(&self, _app: &mut App) {
    }
}

pub fn gui_runner<T>(neat_trainer_host: NeatTrainerHost, trainer_host_client: NeatTrainerHostClient, simulation_renderer: T) where T: bevy::app::Plugin{
    let mut main_app = App::new();

    let sub_app = main_app.add_sub_app("sub", App::new(), |_w, _app|{});
    sub_app
    .add_plugin(NeatSettingsGui)
    .add_plugin(NetworkDefinitionRenderer::default())
    .add_plugin(simulation_renderer)
    .insert_non_send_resource(trainer_host_client)
    .insert_resource(NeatTrainerState{
        run_until: Some(0),
        best_member_so_far: None,
        configuration: neat_trainer_host.initial_configuration.clone(),
        current_generation: 0,
        last_ten_thousand_generations_stats: Vec::new(),
        reset_requested: false,
        species_list: SpeciesList { species_models: Vec::new() },
        config_regulators: neat_trainer_host.initial_config_regulators,
        generic_operations_queue: Vec::new()
    })
    .add_system(client_update_system);

    main_app.run();
}

fn client_update_system(trainer_client: NonSend<NeatTrainerHostClient>,mut trainer_state: ResMut<NeatTrainerState>){
    for event in trainer_client.from_host_receiver.try_iter(){
        match event {
            FromHostEvent::BestNewGenome(best_new) => {
                trainer_state.best_member_so_far = Some(best_new);
            },
            FromHostEvent::ConfigUpdate(config) => {
                trainer_state.configuration = config;
            },
            FromHostEvent::RegulatorUpdate(regulators) => {
                trainer_state.config_regulators = regulators;
            },
            FromHostEvent::GenerationChange(new_gen) => {
                trainer_state.current_generation = new_gen;
                let _ = trainer_client.to_host_sender.send(ToHostEvents::RequestRunStats());
            },
            FromHostEvent::RunStats(new_stats) => {
                trainer_state.last_ten_thousand_generations_stats = new_stats.last_ten_thousand_generations_stats;
                trainer_state.species_list = new_stats.species_list;
            },
            FromHostEvent::HitSuccessThreshold(gen) => {
                trainer_state.run_until = Some(gen);
            },
            FromHostEvent::SetRunUntil(gen) => {
                trainer_state.run_until = Some(gen);
            },
        }
    }

    let _ = trainer_client.to_host_sender.send(ToHostEvents::SetRunUntil(trainer_state.run_until));

    if trainer_state.reset_requested{
        let _ = trainer_client.to_host_sender.send(ToHostEvents::Reset());
        trainer_state.best_member_so_far = None;
        trainer_state.current_generation = 0;
        trainer_state.reset_requested = false;
    }

    let _ = trainer_client.to_host_sender.send(ToHostEvents::UpdateConfig(trainer_state.configuration.clone()));
    let _ = trainer_client.to_host_sender.send(ToHostEvents::UpdateConfigRegulators(trainer_state.config_regulators.clone()));

    for generic_operations in &trainer_state.generic_operations_queue{
        let _ = trainer_client.to_host_sender.send(ToHostEvents::GenericOperation(generic_operations.clone()));
    }
    trainer_state.generic_operations_queue.clear();

}

pub fn render_network_definition(network_definition: Arc<NeatGenome>) {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(NetworkDefinitionRenderer::for_genome_network(network_definition))
    .add_plugin(EguiPlugin)
    .run();
}