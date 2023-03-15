use bevy::{prelude::{Plugin, App, Commands, Query, With, Transform, Mesh, ResMut, Assets, StandardMaterial, Camera, Vec3, Camera3dBundle, Res, DirectionalLightBundle, DirectionalLight, Handle, AssetServer, Without}, render::{view::{RenderLayers}}, time::{Time, Timer}};
use crossbeam_channel::Receiver;
use neatlib::{neat::genome::neat::NeatGenome, phenome::Phenome, renderer::renderer::NeatTrainerState};
use rapier3d::prelude::{Isometry, Real};
use bevy_rapier3d::{utils};
use crate::{simulator::simulation::Simulation, setup_simulation, objects::{self, rocket::{RocketComponent}, floor::Floor, target::{TargetComponent}}, run_step, STEPS, RENDER_STEPS_PER_SECOND};
use bevy::gltf::Gltf;
const RENDER_LAYER: u8 = 1;

pub struct SimulationRender;
struct MyAssetPack(Handle<Gltf>);
impl Plugin for SimulationRender{
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup);
        app.add_system(update_render);
        app.add_system(update_network_to_show);
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}


fn startup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut standard_materials: ResMut<Assets<StandardMaterial>>,ass: Res<AssetServer>){
    let gltf = ass.load("gltf/bone/scene.gltf");
    commands.insert_resource(MyAssetPack(gltf));

    let (sim, rocket, target, event_receiver)  = setup_simulation();

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 400.0, 400.0).looking_at(Vec3::ZERO, Vec3::Y),
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

    //Rocket
    rocket.bevy_setup(&mut commands, &mut meshes, &mut standard_materials, RENDER_LAYER);

    //target  
    target.bevy_setup(&mut commands, &mut meshes, &mut standard_materials, RENDER_LAYER);

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
        rocket,
        target: target,
        event_receiver: event_receiver
    });
}

struct SimulationRun{
    pub network: Option<NeatGenome>,
    pub phenome: Option<Phenome>,
    pub simulation: Simulation,
    pub simulation_complete: bool,
    pub simulation_steps: u32,
    pub current_step: u32,
    pub timer: Timer,
    pub rocket: objects::rocket::Rocket,
    pub target: objects::target::Target,
    pub event_receiver: Receiver<rapier3d::geometry::CollisionEvent>
}

fn update_network_to_show(trainer_state: ResMut<NeatTrainerState>, mut current_run: ResMut<SimulationRun>){
    if trainer_state.best_member_so_far.is_some() && current_run.simulation_complete{
            let x = &trainer_state.best_member_so_far.as_ref().as_ref().unwrap().genome;
            let phenome = Phenome::from_network_schema(x);
            current_run.network = Some(x.clone());
            current_run.phenome = Some(phenome);
            current_run.simulation_complete = false;
    }
}

fn update_render(
    time: Res<Time>, 
    mut nodes_query: Query<(&mut Transform, With<RocketComponent>)>, 
    mut target_query: Query<(&mut Transform, With<TargetComponent>, Without<RocketComponent>)>,
    mut current_run: ResMut<SimulationRun>){
        current_run.timer.tick(time.delta());

        if current_run.timer.just_finished() && current_run.phenome.is_some() {
            step(&mut nodes_query, &mut current_run, &mut target_query);
        }
}

pub fn iso_to_transform(iso: &Isometry<Real>, physics_scale: Real) -> Transform {
    Transform {
        translation: (iso.translation.vector * physics_scale).into(),
        rotation: iso.rotation.into(),
        ..Default::default()
    }
}

fn step(nodes_query: &mut Query<(&mut Transform, With<RocketComponent>)>, current_run: &mut ResMut<SimulationRun>, target_query: &mut Query<(&mut Transform, With<TargetComponent>, Without<RocketComponent>)>){
    let current_step = current_run.current_step; 
    if current_run.current_step >= current_run.simulation_steps{
        current_run.simulation_complete = true;
        current_run.current_step = 0;
        let context = current_run.as_mut();
        let (sim, rocket, target, event_receiver)  = setup_simulation();
        let target_position = target.get_position(&mut context.simulation.rapier_context);
        for (mut transform,_, _) in target_query{
            *transform = utils::iso_to_transform(&target_position, 1.0)
        }
        current_run.simulation = sim;
        current_run.rocket = rocket;
        current_run.target = target;
        current_run.event_receiver = event_receiver
    }else{
        current_run.current_step += 1;
        
        let phenome =  &current_run.phenome.as_ref().unwrap().clone();
        let mut rocket = current_run.rocket.clone();
        let target = current_run.target.clone();
        let event_receiver = current_run.event_receiver.clone();
        let context = current_run.as_mut();

        run_step(&mut rocket,&target, phenome, &mut context.simulation, current_step as f32 / STEPS as f32, event_receiver);
        
        let rocket_position = rocket.get_position(&mut context.simulation.rapier_context);
        for (mut transform, _) in nodes_query.iter_mut(){
            *transform = utils::iso_to_transform(&rocket_position, 1.0)
        }
        let target_position = target.get_position(&mut context.simulation.rapier_context);
        for (mut transform,_, _) in target_query{
            *transform = utils::iso_to_transform(&target_position, 1.0)
        }
        current_run.rocket = rocket;
    }
}