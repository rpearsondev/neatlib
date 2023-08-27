pub mod simulator;
pub mod objects;
pub mod simulation_renderer;
pub mod plugins;
pub mod behaviour_analyser;
use std::{ net::SocketAddr};
use behaviour_analyser::Behaviours;
use bevy::{prelude::App, DefaultPlugins};
use clap::{arg, command, Parser};
use neatlib::{phenome::Phenome, neat::trainer::{fitness::{fitness_resolver::FitnessResolver, fitness_setter::FitnessSetter}, configuration::{Configuration}, node_conf::NodeConf, neat_trainer::NeatTrainer, activation_strategies::activation_strategies::ActivationStrategies, neat_trainer_host::neat_trainer_host::NeatTrainerHost}, activation_functions::ActivationFunction, renderer::{renderer::{self}}, common::{cpu_limiter::CpuLimiter, NeatFloat}, distributed_compute::{distributed_run::DistributedRun, distributed_agent::DistributedAgent, distributed_config::DistributedConfig}};
use objects::{body::Body as SimulationBody, floor::Floor, motor_positions::MotorPositions};
use rapier3d::{crossbeam, prelude::{CollisionEvent, ChannelEventCollector}};
use simulation_renderer::SimulationRender;
use simulator::{simulation::Simulation, rapier_context::RapierContext};

const STEPS: u32 = 1500;
const RENDER_STEPS_PER_SECOND: f32 = 1000.0;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   #[arg(short, long)]
   pub host_address: Option<SocketAddr>,

   #[arg(short, long)]
   pub is_host: bool,

   #[arg(short, long)]
   pub use_testbed: bool
}

//cargo run --release -- --host-address 127.0.0.1:7110 --is-host
// or
// cargo run --release
fn main() {
    let args = Args::parse();
  
    if args.use_testbed {
        simulation_testbed();
        return;
    }
    
    if args.host_address.is_some(){
        let agent_controller = DistributedAgent::new(
            args.host_address.unwrap(),
            args.is_host,
            DistributedConfig::new()
        );
    
        DistributedRun::new(
            agent_controller,
            run_simulation_headless,
            ||{host_setup(true)}
        ).run();
    }else{
        host_setup(false)
    }
}


pub fn host_setup(is_distributed: bool){
    let configuration = Configuration::neat(NodeConf::simple(15, 8),100000.0)
    .genome_minimal_genes_to_connect_ratio(0.0)
    .speciation_use_best_seed_bank(Some(2))
    .speciation_cross_species_reproduction_scale(0.02)
    .mutation_node_available_activation_functions(ActivationFunction::all())
    .mutation_connection_allow_recurrent(true)
    .speciation_drop_species_no_improvement_generations(12)
    .speciation_new_species_protected_for_generations(3)
    .population_size(1000)
    .run_name("locomotion".to_string())
    .target_species(40);

    let (host, client) = NeatTrainerHost::new(NeatTrainer::new(configuration), move |trainer| {
        if is_distributed{
            let mut strategy = ActivationStrategies::get_cpu_distibuted(trainer);
            let mut fitness_setter = FitnessSetter::new();
            strategy.new_generation();
            strategy.compute(&mut fitness_setter);
            fitness_setter.commit(trainer);
        }else{
            let mut strategy = ActivationStrategies::get_cpu_parallel(trainer);
            let mut fitness_setter = FitnessSetter::new();
            strategy.new_generation();
            strategy.compute(run_simulation_headless, &mut fitness_setter);
            fitness_setter.commit(trainer);
        }
    });
    
    renderer::gui_runner(host, client, SimulationRender{ test_bed_mode: false });
}

pub fn simulation_testbed(){
    let mut main_app = App::new();
    main_app.add_plugins(DefaultPlugins);
    main_app.add_plugin(SimulationRender{ test_bed_mode: true});
    main_app.run()
}

pub fn run_simulation_headless(phenotype: &dyn Phenome, fitness_resolver: &mut FitnessResolver) {
    let (mut simulation, mut body, collision_recv) = setup_simulation();
    
    let mut previous_sensors = body.get_sensors(&mut simulation.rapier_context).unwrap();
    let mut left_leg_high = 0;
    let mut right_leg_high = 0;
    for step in 0..STEPS{
        run_step(&mut body,  phenotype, &mut simulation,  step as f32 / STEPS as f32, collision_recv.clone());
        let sensors = body.get_sensors(&mut simulation.rapier_context).unwrap();

        CpuLimiter::limit(0.95);

        let behaviours = Behaviours::from_sensors(sensors, previous_sensors);

        if behaviours.has_fallen_over{
            break;
        }else{
            fitness_resolver.add_reward(11, 0.1);
        }
        if sensors.leg_left_lower_sensors.translation_y > 3.0 || sensors.leg_right_lower_sensors.translation_y > 3.0{
            fitness_resolver.add_reward(13, 0.1);
        }
        if behaviours.has_moved_quite_quick{
            fitness_resolver.add_reward(14, 0.1);
        }
        if behaviours.is_upright{
            fitness_resolver.add_reward(15, 1.0);
        }
        if behaviours.has_feet_in_alternate_positions{
            fitness_resolver.add_reward(16, 0.1);
        }
        if behaviours.both_legs_are_moving{
            fitness_resolver.add_reward(17, 0.1);
        }
        if behaviours.is_left_knee_high{
            fitness_resolver.add_reward(18, 0.1);
            left_leg_high +=1;
        }
        if behaviours.is_right_knee_high{
            fitness_resolver.add_reward(19, 0.1);
            right_leg_high +=1;
        }
        previous_sensors = sensors;
    }

    let sensors = body.get_sensors(&mut simulation.rapier_context).unwrap();
    let behaviours = Behaviours::from_sensors(sensors, previous_sensors);
    fitness_resolver.add_novelty_component(0, sensors.torso_sensors.translation_x, 10);
    fitness_resolver.add_novelty_component(1, sensors.torso_sensors.translation_y,  10);
    fitness_resolver.add_objective_fitness_component_with_novelty(2, 1.0, -200.0, sensors.torso_sensors.translation_z, 1);
    fitness_resolver.add_objective_fitness_component(3, 0.2, 0.0, behaviours.distance_from_upright);
    fitness_resolver.add_novelty_component(6, left_leg_high as NeatFloat, 1);
    fitness_resolver.add_novelty_component(7, right_leg_high as NeatFloat, 1);
}

pub fn setup_simulation() -> (Simulation, SimulationBody, crossbeam_channel::Receiver<CollisionEvent>) {
    let (collision_send, collision_recv) = crossbeam::channel::unbounded();
    let (contact_force_send, _) = crossbeam::channel::unbounded();
    let event_handler = ChannelEventCollector::new(collision_send, contact_force_send);

    let mut simulation = Simulation{rapier_context: RapierContext {event_handler: Some(event_handler), ..Default::default() } ,..Default::default()};

    simulation.gravity(true);
    simulation.add_object(Floor::new());
    let body = simulation.add_object(SimulationBody::new());
    (simulation, body, collision_recv)
}

pub fn run_step(body: &mut SimulationBody, phenome: &dyn Phenome, simulation: &mut Simulation, sim_time_fraction: f32, _collision_recv: crossbeam_channel::Receiver<CollisionEvent>){
    let sensors = body.get_sensors(&mut simulation.rapier_context).unwrap();
    
    let result = phenome.activate(
        &vec![
        
            sensors.left_hip_forward_joint.motor_position,
            sensors.left_hip_outward_joint.motor_position,
            sensors.left_knee_joint.motor_position,

            sensors.right_hip_forward_joint.motor_position,
            sensors.right_hip_outward_joint.motor_position,
            sensors.right_knee_joint.motor_position,

            sensors.counter_balance_z_joint.motor_position,
            sensors.counter_balance_x_joint.motor_position,
            sensors.torso_sensors.y_distance_in_radians_from_y_axis,
            sensors.torso_sensors.x_distance_in_radians_from_y_axis,
            sensors.torso_sensors.z_distance_in_radians_from_y_axis,
            (sim_time_fraction * 12.0).sin(),
            (sim_time_fraction * 8.0).sin(),
            (sim_time_fraction * 4.0).sin(),
            (sim_time_fraction * 2.0).sin(),
   
        ]
    );
        body.set_motor_positions(&mut simulation.rapier_context, MotorPositions { 
        
        counter_balance_z_position: join_output_clamp(result[0], sensors.counter_balance_z_joint.motor_position, sensors.counter_balance_z_joint.min_limit, sensors.counter_balance_z_joint.max_limit),
        counter_balance_x_position: join_output_clamp(result[1], sensors.counter_balance_x_joint.motor_position, sensors.counter_balance_x_joint.min_limit, sensors.counter_balance_x_joint.max_limit),
     
        left_hip_forward_axis_position: join_output_clamp(result[2], sensors.left_hip_forward_joint.motor_position, sensors.left_hip_forward_joint.min_limit, sensors.left_hip_forward_joint.max_limit),
        left_hip_outward_axis_position: join_output_clamp(result[3], sensors.left_hip_outward_joint.motor_position, sensors.left_hip_outward_joint.min_limit,sensors.left_hip_outward_joint.max_limit),
        left_knee_position: join_output_clamp(result[4], sensors.left_knee_joint.motor_position, sensors.left_knee_joint.min_limit, sensors.left_knee_joint.max_limit),
       
        right_hip_forward_axis_position: join_output_clamp(result[5], sensors.right_hip_forward_joint.motor_position, sensors.right_hip_forward_joint.min_limit, sensors.right_hip_forward_joint.max_limit),
        right_hip_outward_axis_position: join_output_clamp(result[6], sensors.right_hip_outward_joint.motor_position, sensors.right_hip_outward_joint.min_limit, sensors.right_hip_outward_joint.max_limit),
        right_knee_position: join_output_clamp(result[7], sensors.right_knee_joint.motor_position, sensors.right_knee_joint.min_limit, sensors.right_knee_joint.max_limit),
    });
    simulation.step();
}

fn join_output_clamp(mut output_value: f32 ,motor_position: f32, min_limit: f32, max_limit: f32) -> f32{
    output_value = output_value.clamp(min_limit + 0.01, max_limit - 0.01);
    output_value = output_value.clamp(motor_position- 0.10, motor_position + 0.10);
    output_value
}