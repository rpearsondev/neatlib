
use std::sync::Arc;

use bevy::{prelude::{*}, time::FixedTimestep, render::{camera::RenderTarget, view::RenderLayers}};
use bevy_egui::*;
use wgpu::{TextureDescriptor, TextureDimension, TextureUsages, TextureFormat, Extent3d};
use crate::{neat::{genome::neat::NeatGenome}, renderer::{plugins::{neat_settings_gui::NeatSettingsGuiState, network_lines::network_lines::{LineMaterial, LineList}}, renderer::NeatTrainerState}};
use super::diagram::{Diagram, DiagramNode, DiagramConnection};

const RENDER_LAYER:u8 = 6;

pub struct NetworkDefinitionRenderer{
    network: Option<Arc<NeatGenome>>
}

pub struct NetworkDefinitionRendererImageHandle{
    pub image_handle: Option<Handle<Image>>
}


#[derive(Component)]
struct NetworkEntityCamera;


impl Plugin for NetworkDefinitionRenderer {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.network.clone());
        app.insert_resource(NetworkDefinitionRendererImageHandle{image_handle: None});
        app.add_startup_system(setup);
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.5))
                .with_system(draw_network)
        );
        app.add_system(rotate_camera);
        app.add_system(network_window);
        app.add_system(set_network_resources);
        app.add_plugin(MaterialPlugin::<LineMaterial>::default());
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}
impl NetworkDefinitionRenderer{
    pub fn for_genome_network(network: Arc<NeatGenome>) -> Self {
        Self {
           network: Some(network)
        }
    }
    pub fn default() -> Self {
        Self {
           network: None
        }
    }
}

fn set_network_resources(
    trainer_state: ResMut<NeatTrainerState>,
    mut network: ResMut<Option<Arc<NeatGenome>>>,
    state: ResMut<NeatSettingsGuiState>,
    mut command: Commands){
    let best_so_far = &trainer_state.best_member_so_far;
    if best_so_far.is_some() && (state.show_network || state.show_substrate){
        let member = best_so_far.as_ref().unwrap();
        let mut genome = member.genome.clone();
        genome.genes.cleanup_orphan_nodes();

     
        if state.show_network{
            network.replace(Arc::new(genome));
        }else if network.is_some(){
            command.insert_resource(None as Option<Arc<NeatGenome>>);
        }
    }
}

fn network_window(
    mut egui_ctx: ResMut<EguiContext>,
    mut state: ResMut<NeatSettingsGuiState>,
    network_definition_renderer: Res<NetworkDefinitionRendererImageHandle>
) {
    if network_definition_renderer.image_handle.is_some() {
        let image_handle = network_definition_renderer.image_handle.as_ref().unwrap().clone();
        let texture_id = egui_ctx.add_image(image_handle.clone_weak());
        egui::Window::new("network")
        .vscroll(true)
        .default_size(egui::Vec2{x: 800.0, y: 600.0})
        .open(&mut state.show_network)
        .show(egui_ctx.ctx_mut(), |ui| {
            let available_size = ui.available_size();
            ui.image(texture_id, available_size);
        });
    }
}

fn rotate_camera(time: Res<Time>,mut q: Query<&mut Transform, With<NetworkEntityCamera>> ){
    for mut transform in q.iter_mut() {
        transform.translate_around(Vec3::ZERO, Quat::from_rotation_z(0.3 * time.delta_seconds()));
        transform.look_at(Vec3::ZERO, Vec3::Z);
    }
}

#[derive(Component)]
struct NetworkRenderComponent;

struct DrawingContext<'a>{
    commands: Commands<'a, 'a>,
    meshes: ResMut<'a, Assets<Mesh>>,
    standard_materials: ResMut<'a,  Assets<StandardMaterial>>,
    line_materials: ResMut<'a,  Assets<LineMaterial>>
}

fn draw_network(
    network: Res<Option<Arc<NeatGenome>>>,
    commands: Commands,
    _asset_server: Res<AssetServer>,
    meshes: ResMut<Assets<Mesh>>,
    standard_materials: ResMut<Assets<StandardMaterial>>,
    line_materials: ResMut<Assets<LineMaterial>>,
    mut nodes_query: Query<(Entity, With<NetworkRenderComponent>)>
){
    if !network.is_changed() {
        return;
    }

    let mut context = DrawingContext{
        commands: commands,
        standard_materials,
        line_materials: line_materials,
        meshes: meshes
    };

    for (entity, _) in nodes_query.iter_mut(){
        context.commands.entity(entity).despawn_recursive();
    }

    if network.is_none() {
        return;
    }

    let diagram = Diagram::from(&network.as_ref().as_ref().unwrap());
  
    for node in diagram.nodes{
        draw_neuron(node, &mut context);
    }

    for connection in diagram.connections{
        draw_connection(connection, &mut context);
    }
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut network_definition_renderer_image_handle: ResMut<NetworkDefinitionRendererImageHandle>
){

    let size = Extent3d {
        width: 1024,
        height: 768,
        ..default()
    };

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    }).insert(RenderLayers::layer(RENDER_LAYER));


    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);

    network_definition_renderer_image_handle.image_handle = Some(image_handle.clone_weak());

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Z),
        camera: Camera{
            priority: 0,
            target: RenderTarget::Image(image_handle),
            ..Default::default()
        },
        ..default()
    }).insert(RenderLayers::layer(RENDER_LAYER)).insert(NetworkEntityCamera{});
}

fn draw_neuron(node: DiagramNode ,ctx: &mut DrawingContext){
    ctx.commands.spawn_bundle(PbrBundle {
        mesh: ctx.meshes.add(Mesh::from(shape::Cube { size: node.size })),
        material:  ctx.standard_materials.add(node.color.into()),
        transform: Transform::from_xyz(node.position.x, node.position.y, node.position.z),
        ..default()
    }).insert(RenderLayers::layer(RENDER_LAYER)).insert(NetworkRenderComponent{});
}

fn draw_connection(connection: DiagramConnection, ctx: &mut DrawingContext){
        ctx.commands.spawn_bundle(
            MaterialMeshBundle {
            mesh: ctx.meshes.add(Mesh::from(LineList {

                lines: vec![
                    (connection.from_position, connection.to_position),
                ],
            })),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            material: ctx.line_materials.add(LineMaterial {
                color: connection.color
            }),
            ..default()
        }).insert(RenderLayers::layer(RENDER_LAYER)).insert(NetworkRenderComponent{});
    
}
