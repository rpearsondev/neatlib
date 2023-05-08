use bevy::{prelude::{Plugin, App, Commands, Query, Transform, Mesh, ResMut, Assets, StandardMaterial, Camera, Vec3, Camera3dBundle, Res, DirectionalLightBundle, DirectionalLight}, render::{view::{RenderLayers}}, time::{Time, Timer}};
use bevy_egui::EguiPlugin;
use crossbeam_channel::Receiver;
use neatlib::{neat::genome::neat::NeatGenome, renderer::renderer::NeatTrainerState, cpu_phenome::CpuPhenome};
use rapier3d::prelude::{Isometry, Real};
use crate::{simulator::simulation::Simulation, setup_simulation, objects::{self, body::{ Body, BodyPartUpdateComponent}, floor::Floor, motor_positions::MotorPositions}, run_step, STEPS, RENDER_STEPS_PER_SECOND, plugins::body_window::body_window::BodyWindow, behaviour_analyser::Behaviours};
const RENDER_LAYER: u8 = 1;

pub struct SimulationRender{
    pub test_bed_mode: bool
}

impl Plugin for SimulationRender{
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup);
    
        app.add_plugin(BodyWindow{});
        if self.test_bed_mode{
            app.add_plugin(EguiPlugin);
            app.add_system(test_bed_update_render);
        }else{
            app.add_system(update_render);
            app.add_system(update_network_to_show);
        }
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn startup(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut standard_materials: ResMut<Assets<StandardMaterial>>){
    let (sim, body, event_receiver)  = setup_simulation();
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(8.0, 20.0, 60.0).looking_at(Vec3::ZERO, Vec3::Y),
        camera: Camera{
            priority: 0,
            ..Default::default()
        },
        ..Default::default()
    }).insert(RenderLayers::from_layers(&[RENDER_LAYER,0]));

    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight{
            illuminance: 5000.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 300.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..Default::default()
    }).insert(RenderLayers::layer(RENDER_LAYER));

    //Ball
    body.bevy_setup(&mut commands, &mut meshes, &mut standard_materials, RENDER_LAYER);

    //floor
    Floor::bevy_setup(&mut commands, &mut meshes, &mut standard_materials, RENDER_LAYER);

    commands.insert_resource(SimulationRun{
        network: None,
        phenome: None,
        simulation_steps: STEPS,
        current_step: 0,
        simulation_complete: true,
        timer: Timer::from_seconds(1.0 / RENDER_STEPS_PER_SECOND, true),
        simulation: sim,
        body,
        event_receiver: event_receiver
    });
}

pub struct SimulationRun{
    pub network: Option<NeatGenome>,
    pub phenome: Option<CpuPhenome>,
    pub simulation: Simulation,
    pub simulation_complete: bool,
    pub simulation_steps: u32,
    pub current_step: u32,
    pub timer: Timer,
    pub body: objects::body::Body,
    pub event_receiver: Receiver<rapier3d::geometry::CollisionEvent>
}

fn update_network_to_show(trainer_state: ResMut<NeatTrainerState>, mut current_run: ResMut<SimulationRun>){
    if trainer_state.best_member_so_far.is_some() && current_run.simulation_complete{
        let x = &trainer_state.best_member_so_far.as_ref().as_ref().unwrap().genome;
        let phenome = CpuPhenome::from_network_schema(x);
        current_run.network = Some(x.clone());
        current_run.phenome = Some(phenome);
        current_run.simulation_complete = false;
    }
}

fn update_render(
    time: Res<Time>, 
    mut bodypart_update_query: Query<(&mut Transform, &BodyPartUpdateComponent)>,
    mut current_run: ResMut<SimulationRun>){
        current_run.timer.tick(time.delta());

        if current_run.timer.just_finished() && current_run.phenome.is_some() {
            step(&mut bodypart_update_query, &mut current_run);
        }
}

fn test_bed_update_render(
    time: Res<Time>, 
    mut bodypart_update_query: Query<(&mut Transform, &BodyPartUpdateComponent)>,
    mut current_run: ResMut<SimulationRun>,
    motor_positions: Res<MotorPositions>){
    current_run.timer.tick(time.delta());

    test_bed_step(
        &mut bodypart_update_query, 
        &mut current_run,
        motor_positions
    );
}

pub fn iso_to_transform(iso: &Isometry<Real>, physics_scale: Real) -> Transform {
    Transform {
        translation: (iso.translation.vector * physics_scale).into(),
        rotation: iso.rotation.into(),
        ..Default::default()
    }
}

fn step (
    body_part_update_query: &mut Query<(&mut Transform, &BodyPartUpdateComponent)>, 
    current_run: &mut ResMut<SimulationRun>){
    let current_step = current_run.current_step; 
    if current_run.current_step >= current_run.simulation_steps{
        current_run.simulation_complete = true;
        current_run.current_step = 0;
        let (sim, body, event_receiver)  = setup_simulation();
        current_run.simulation = sim;
        current_run.body = body;
        current_run.event_receiver = event_receiver;
    }else{
        current_run.current_step += 1;
        let phenome =  &current_run.phenome.as_ref().unwrap().clone();
        let mut body = current_run.body.clone();
        let event_receiver = current_run.event_receiver.clone();
        let context = current_run.as_mut();

        let sensors = body.get_sensors(&mut context.simulation.rapier_context).unwrap();
        let behaviours = Behaviours::from_sensors( sensors, sensors);
        if behaviours.has_fallen_over{
            context.simulation_complete = true;
            context.current_step = 0;
            let (sim, body, event_receiver)  = setup_simulation();
            context.simulation = sim;
            context.body = body;
            context.event_receiver = event_receiver;
        }

        run_step(&mut body, phenome, &mut context.simulation, current_step as f32 / STEPS as f32, event_receiver);
        
        Body::bevy_update(context, body_part_update_query);

        current_run.body = body;
    }
}

fn test_bed_step(
    body_part_update_query: &mut Query<(&mut Transform, &BodyPartUpdateComponent)>, 
    current_run: &mut ResMut<SimulationRun>,
    motor_positions: Res<MotorPositions>
){
    let _event_receiver = current_run.event_receiver.clone();
    let simuation_run = current_run.as_mut();

    simuation_run.body.set_motor_positions(&mut simuation_run.simulation.rapier_context, *motor_positions);

    simuation_run.simulation.step();
    Body::bevy_update(simuation_run, body_part_update_query);
}