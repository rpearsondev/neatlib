pub mod simulator;
pub mod objects;
pub mod simulation_renderer;

use bevy_rapier3d::{na::{Vector3, UnitQuaternion}};
use neatlib::{phenome::Phenome, neat::trainer::{fitness::{fitness_resolver::FitnessResolver, fitness_setter::FitnessSetter}, configuration::{Configuration, OffSpringMode}, node_conf::NodeConf, neat_trainer::NeatTrainer, activation_strategies::activation_strategies::ActivationStrategies, neat_trainer_host::neat_trainer_host::NeatTrainerHost}, activation_functions::ActivationFunction, renderer::renderer::{self}, common::{NeatFloat, cpu_limiter::CpuLimiter}, hyperneat::substrate::{substrate_type::SubstrateType, substrate_coordinate_scheme::SubstrateCoordinateScheme, substrate_geometric_organization::SubstrateGeometricOrganization, substrate_set_connection_mode::SubstrateSetConnectionMode, substrate_set_cppn_mode::SubstrateSetCPPNMode, substrate::Substrate, substrate_set::SubstrateSet}};
use objects::{rocket::Rocket as SimulationRocket, floor::Floor, target::Target};
use rapier3d::{crossbeam, prelude::{CollisionEvent, ChannelEventCollector}};
use simulation_renderer::SimulationRender;
use simulator::{simulation::Simulation, rapier_context::RapierContext};

const STEPS: u32 = 1000;
const RENDER_STEPS_PER_SECOND: f32 = 500.0;

fn main() {

    let _hyperneat_configuration = Configuration::hyperneat(
        SubstrateSet::new(
            SubstrateCoordinateScheme::CenterOut,
            SubstrateGeometricOrganization::Sandwich,
            ActivationFunction::TANH,
            SubstrateSetConnectionMode::ForwardOne,
            SubstrateSetCPPNMode::XyzAngleDistanceToXyzAngleDistance,
        Substrate::new(SubstrateType::Input, 9), 
    vec![
            Substrate::new(SubstrateType::Hidden, 3),
        ],
        Substrate::new(SubstrateType::Output, 2)
    ), 1000000.0)
    .target_species(8)
    .genome_minimal_genes_to_connect_ratio(1.0)
    .mutation_node_available_activation_functions(ActivationFunction::for_cppn())
    .mutation_connection_allow_recurrent(false)
    .speciation_drop_species_no_improvement_generations(8)
    .speciation_new_species_protected_for_generations(3)
    .population_size(200);

    let configuration = Configuration::neat(NodeConf::simple(9, 2), 100000.0)
    .target_species(60)
    .run_name("rocket".to_string())
    .genome_minimal_genes_to_connect_ratio(0.0)
    .mutation_node_available_activation_functions(ActivationFunction::all())
    .mutation_connection_allow_recurrent(false)
    .speciation_offspring_mode(OffSpringMode::AdjustedMemberRange)
    .speciation_drop_species_no_improvement_generations(8)
    .speciation_new_species_protected_for_generations(4)
    .speciation_cross_species_reproduction_scale(0.01)
    .population_size(1000);

    let (host, client) = NeatTrainerHost::new(NeatTrainer::new(configuration), move |trainer| {
        let mut strategy = ActivationStrategies::get_cpu_parallel(trainer);
        let mut fitness_setter = FitnessSetter::new();
        strategy.new_generation();
        strategy.compute(run_simulation_headless, &mut fitness_setter);
        fitness_setter.commit(trainer);
    });
    
    renderer::gui_runner(host, client, SimulationRender);
}

pub fn run_simulation_headless(phenotype: &Phenome, fitness_resolver: &mut FitnessResolver) {
    let (mut simulation, mut rocket, target, collision_recv) = setup_simulation();
    let mut total_x_distances = 0.0;
    let mut total_y_distances = 0.0;
    let mut total_y_axis_distances = 0.0;
    let mut largest_distance_away_from_target = 0.0;
    let target_position = target.get_position(&mut simulation.rapier_context);
    let mut distance_into_sim = 0.0;

    for step in 0..STEPS{
        run_step(&mut rocket, &target, phenotype, &mut simulation,  step as f32 / STEPS as f32, collision_recv.clone());

        //limit the simulation to 99% cpu to allow the UI to continue to function
        CpuLimiter::limit(0.99);

        let rocket_position = rocket.get_position(&mut simulation.rapier_context);
        let x_diff = f32::abs(target_position.translation.x - rocket_position.translation.x);
        let y_diff = f32::abs(target_position.translation.y - rocket_position.translation.y);
        total_x_distances += x_diff;
        total_y_distances += y_diff;
        let distance_from_target = f32::sqrt(x_diff * y_diff);
        if distance_from_target > largest_distance_away_from_target{
            largest_distance_away_from_target = distance_from_target;
        }

        let angle_from_y = rocket_position.rotation.angle_to(&UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.0));
        total_y_axis_distances += angle_from_y;

        if angle_from_y.abs() > 1.0{
            fitness_resolver.add_punishment(11, 10000.0);
            break;
        }
        if rocket_position.translation.y < 20.0{
            fitness_resolver.add_punishment(12, 5000.0);
            break;
        }
        if rocket.collisions > 0 {
            fitness_resolver.add_reward(10, 12000.0 * (1.0 - distance_into_sim) as NeatFloat);
            fitness_resolver.add_reward(10, 6000.0);
            total_x_distances = 0.0;
            total_y_distances = 0.0;
            largest_distance_away_from_target = 0.0;
            rocket.collisions = 0;
            total_y_axis_distances = 0.0;
        }
        distance_into_sim = step as NeatFloat / STEPS as NeatFloat;
    }
    let final_rocket_position = rocket.get_position(&mut simulation.rapier_context);
    let final_rocket_speed = rocket.get_speed(&mut simulation.rapier_context);
    let final_angle_from_y = final_rocket_position.rotation.angle_to(&UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.0));
    fitness_resolver.add_novelty_component(1, final_rocket_position.translation.x, 5);
    fitness_resolver.add_novelty_component(2, final_rocket_position.translation.y, 5);
    fitness_resolver.add_objective_fitness_component_with_novelty(3, 5.25, 0.0, total_x_distances / STEPS as f32, 5);
    fitness_resolver.add_objective_fitness_component_with_novelty(4, 1.25, 0.0, total_y_distances / STEPS as f32, 5);
    fitness_resolver.add_novelty_component(5, largest_distance_away_from_target, 5);
    fitness_resolver.add_novelty_component(6, final_rocket_speed, 5);
    fitness_resolver.add_objective_fitness_component_with_novelty(7, 0.01, 0.0, total_y_axis_distances / STEPS as f32, 5);
    fitness_resolver.add_novelty_component(8, final_angle_from_y, 5);
}

pub fn setup_simulation() -> (Simulation, SimulationRocket, Target, crossbeam_channel::Receiver<CollisionEvent>) {
    let (collision_send, collision_recv) = crossbeam::channel::unbounded();
    let (contact_force_send, _) = crossbeam::channel::unbounded();
    let event_handler = ChannelEventCollector::new(collision_send, contact_force_send);

    let mut simulation = Simulation{rapier_context: RapierContext {event_handler: Some(event_handler), ..Default::default() } ,..Default::default()};

    simulation.add_gravity();
    simulation.add_object(Floor::new());
    let target = simulation.add_object(Target::new());
    let rocket = simulation.add_object(SimulationRocket::new());
    (simulation, rocket, target, collision_recv)
}

pub fn run_step(rocket: &mut SimulationRocket, target: &Target, phenome: &Phenome, simulation: &mut Simulation, sim_time_fraction: f32, collision_recv: crossbeam_channel::Receiver<CollisionEvent>) {
    let position = rocket.get_position(&mut simulation.rapier_context);
    let target_position = target.get_position(&mut simulation.rapier_context);
    let angle_from_y = position.rotation.angle_to(&UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.0));
    
    let target_vec3 = &Vector3::from_vec(vec![target_position.translation.x, target_position.translation.y, target_position.translation.z]);
    let rocket_vec3 = &Vector3::from_vec(vec![position.translation.x, position.translation.y, position.translation.z]);
    let smallest_angle = rocket_vec3.angle(target_vec3);
    

    let speed = rocket.get_speed(&mut simulation.rapier_context);
    //[-1, -1], [-1, 0], [-1, 1], 
    //[0, -1], [0, 0], [0, 1], 
    //[1, -1], [1, 0], [1, 1]
    let result = phenome.activate(&vec![
        speed, 
        rocket_vec3.metric_distance(target_vec3),
        angle_from_y, 

        (target_position.translation.x - position.translation.x) / 500.0, 
        (position.translation.x - target_position.translation.x) / 500.0, 
        smallest_angle,
        
        
        (position.translation.y - target_position.translation.y) / 500.0, 
        (target_position.translation.y - position.translation.y) / 500.0, 
        
        (sim_time_fraction % 0.01).sin()
        ]);
   
    let left_right = result[0];
    let up = result[1];
    if left_right > 0.5 {
        rocket.boost_right(&mut simulation.rapier_context);
    }
    if left_right < 0.5 {
        rocket.boost_left(&mut simulation.rapier_context);
    }
    if up > 0.01 {
        rocket.boost_up(&mut simulation.rapier_context);
    }

    while let Ok(_collision_event) = collision_recv.try_recv() {
        match _collision_event {
            CollisionEvent::Started(_, _, _) => {
                rocket.collisions += 1;
                target.move_to_elsewhere(&mut simulation.rapier_context)
            },
            CollisionEvent::Stopped(_, _, _) => {

            },
        }
    }

    simulation.step();
}