
use bevy::{prelude::{*}, time::FixedTimestep, render::{camera::RenderTarget, view::RenderLayers}};
use wgpu::{TextureUsages, Extent3d, TextureDimension, TextureDescriptor, TextureFormat};
use crate::{hyperneat::substrate::{substrate_set::SubstrateSet, network_definition_factory::{XyzNetworkParameter, NetworkDefinitionFactory}}, phenome::Phenome, renderer::{plugins::network_lines::network_lines::{LineList, LineMaterial}, renderer::NeatTrainerState}};
use std::{f32::consts::PI};
//use bevy_text_mesh::prelude::*;

use bevy_egui::*;
use crate::renderer::plugins::neat_settings_gui::NeatSettingsGuiState;
use super::diagram::{*};

const RENDER_LAYER:u8 = 5;

#[derive(Component)]
struct SubstrateEntity;

#[derive(Component)]
struct SubstrateEntityCamera;

pub struct SubstrateRenderer{
    substrate_set: Option<SubstrateSet>,
    network_definition_connections: Option<Vec<XyzNetworkParameter>>,
    render_to_image: bool
}

impl Plugin for SubstrateRenderer {
    fn build(&self, app: &mut App) {
        app.insert_resource(SubstrateRendererImageOptions{image_handle: None, render_to_image: self.render_to_image});
        app.insert_resource(self.substrate_set.clone());
        app.insert_resource(self.network_definition_connections.clone());
        app.add_startup_system(setup);
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.5))
                .with_system(draw_substrate)
        );
        app.add_system(rotate_camera);
        
        if !self.substrate_set.is_some(){
            app.add_system(network_window);
            app.add_system(set_network_resources_for_gui);
        }
        
        //app.add_plugin(TextMeshPlugin);
        app.add_plugin(MaterialPlugin::<LineMaterial>::default());
    }
}
impl SubstrateRenderer{
    pub fn for_substrate_set(substrate_set: SubstrateSet) -> Self {
        Self {
            substrate_set: Some(substrate_set),
            network_definition_connections: None,
            render_to_image: false

        }
    }
    pub fn for_substrate_set_with_connections(substrate_set: SubstrateSet, network_definition_connections: Vec<XyzNetworkParameter>) -> Self {
        Self {
            substrate_set: Some(substrate_set),
            network_definition_connections: Some(network_definition_connections),
            render_to_image: false
        }
    }
    pub fn default() -> Self{
        Self {
            substrate_set: None,
            network_definition_connections: None,
            render_to_image: true
        }
    }
}

pub struct SubstrateRendererImageOptions{
    pub image_handle: Option<Handle<Image>>,
    pub render_to_image: bool
}

fn rotate_camera(time: Res<Time>,mut q: Query<&mut Transform, With<SubstrateEntityCamera>>, substrate_set: Res<Option<SubstrateSet>> ){
    if substrate_set.is_none(){
        return;
    }

    for mut transform in q.iter_mut() {
        transform.translate_around(Vec3::ZERO, Quat::from_rotation_z(0.3 * time.delta_seconds()));
        transform.look_at(Vec3::ZERO, Vec3::Z);
    }
}

fn network_window(
    mut egui_ctx: ResMut<EguiContext>,
    mut state: ResMut<NeatSettingsGuiState>,
    network_definition_renderer: Res<SubstrateRendererImageOptions>
) {
    if network_definition_renderer.image_handle.is_some() {
        let image_handle = network_definition_renderer.image_handle.as_ref().unwrap().clone();
        let texture_id = egui_ctx.add_image(image_handle.clone_weak());
        egui::Window::new("substrate")
        .vscroll(true)
        .default_size(egui::Vec2{x: 800.0, y: 600.0})
        .open(&mut state.show_substrate)
        .show(egui_ctx.ctx_mut(), |ui| {
            let available_size = ui.available_size();
            ui.image(texture_id, available_size);
        });
    }
}

fn set_network_resources_for_gui(
    trainer_state: Res<NeatTrainerState>,
    mut substrate: ResMut<Option<SubstrateSet>>,
    mut connections: ResMut<Option<Vec<XyzNetworkParameter>>>,
    state: Res<NeatSettingsGuiState>,
    mut command: Commands){
    let best_so_far = &trainer_state.best_member_so_far;
    if best_so_far.is_some() && (state.show_network || state.show_substrate){
        let member = best_so_far.as_ref().unwrap();
        let genome = member.genome.clone();

        if trainer_state.configuration.hyperneat_substrate_set.is_some(){
            let substrate_set =trainer_state.configuration.hyperneat_substrate_set.clone().unwrap();
            let cppn = Phenome::from_network_schema(&genome);
            let network_parameters = NetworkDefinitionFactory::xyz_mapping(cppn, &substrate_set.cppn_inputs, &substrate_set);
            
            if state.show_substrate {
                connections.replace(network_parameters.connections);
                substrate.replace(substrate_set);
            }else if connections.is_some() || substrate.is_some(){
                command.insert_resource(None as Option<SubstrateSet>);
                command.insert_resource(None as Option<Vec<XyzNetworkParameter>>);
            }
        }
    }else{
        command.insert_resource(None as Option<SubstrateSet>);
        command.insert_resource(None as Option<Vec<XyzNetworkParameter>>);
    }
}

struct DrawingContext<'a>{
    commands: Commands<'a, 'a>,
    meshes: ResMut<'a, Assets<Mesh>>,
    materials: ResMut<'a, Assets<StandardMaterial>>,
    line_materials: ResMut<'a, Assets<LineMaterial>>,
    _asset_server: Res<'a, AssetServer>
}
fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut network_definition_renderer_image_options: ResMut<SubstrateRendererImageOptions>
)
{
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(3.0, 3.0, 3.0),
        ..default()
    }).insert(RenderLayers::layer(RENDER_LAYER));

    let size = Extent3d {
        width: 1024,
        height: 768,
        ..default()
    };

    if network_definition_renderer_image_options.render_to_image {
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

        image.resize(size);

        let image_handle = images.add(image);

        network_definition_renderer_image_options.image_handle = Some(image_handle.clone_weak());
    

        commands.spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(2.0, 2.0, 2.0).looking_at(Vec3::ZERO, Vec3::Z),
            camera: Camera{
                priority: 0,
                target: RenderTarget::Image(image_handle),
                ..Default::default()
            },
            ..default()
        }).insert(RenderLayers::layer(RENDER_LAYER)).insert(SubstrateEntityCamera{});
    } else {
           commands.spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(2.0, 2.0, 2.0).looking_at(Vec3::ZERO, Vec3::Z),
            camera: Camera{
                priority: 0,
                ..Default::default()
            },
            ..default()
        }).insert(RenderLayers::layer(RENDER_LAYER)).insert(SubstrateEntityCamera{});
    }
}

fn draw_substrate(
    substrate_set_opt: Res<Option<SubstrateSet>>,
    connections: Res<Option<Vec<XyzNetworkParameter>>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    line_materials: ResMut<Assets<LineMaterial>>,
    mut substrate_query: Query<(Entity, With<SubstrateEntity>)>
)
{
    if !(substrate_set_opt.is_changed() || connections.is_changed()) {
        return;
    }

    for (entity, _) in substrate_query.iter_mut(){
        commands.entity(entity).despawn_recursive();
    }
    
    if substrate_set_opt.is_none(){
        return;
    }

    let substrate_set = (*substrate_set_opt).as_ref().unwrap();

    let mut context = DrawingContext{
        commands: commands,
        materials: materials,
        meshes: meshes,
        _asset_server: asset_server,
        line_materials: line_materials
    };

    let diagram = Diagram::from(&substrate_set, &connections);
  
    for sheet in diagram.substrates{
        draw_sheet(sheet, &mut context);
    }

    for sheet in diagram.nodes{
        draw_node(sheet, &mut context);
    }

    for annotation in diagram.annotations{
        let transform = Transform::from_xyz(annotation.position.x, annotation.position.z, annotation.position.y);
        draw_text(annotation.text, annotation.size, &transform, annotation.offset_x, annotation.offset_y, &mut context);
    }

    draw_connections(&diagram.connections, &mut context);
}

fn draw_sheet(sheet: DiagramSubstrate ,context: &mut DrawingContext){
    let transform = Transform::from_xyz(0.0, sheet.position.z, 0.0);
    context.commands.spawn_bundle(PbrBundle {
        mesh: context.meshes.add(Mesh::from(shape::Plane { size: sheet.width })),
        material:  context.materials.add(sheet.color.into()),
        transform: transform,
        ..default()
    }).insert(SubstrateEntity{}).insert(RenderLayers::layer(RENDER_LAYER));

    let mut transform_reverse = transform.clone();
    transform_reverse.rotate_local_z(PI);
    
    context.commands.spawn_bundle(PbrBundle {
        mesh: context.meshes.add(Mesh::from(shape::Plane { size: sheet.width })),
        material:  context.materials.add(sheet.color.into()),
        transform: transform_reverse,
        ..default()
    }).insert(SubstrateEntity{}).insert(RenderLayers::layer(RENDER_LAYER));
}
fn draw_node(node: DiagramSubstrateNode ,context: &mut DrawingContext){
    let mut transform = Transform::from_xyz(0.0, 0.0, 0.0);
    transform.rotate_local_x(-(PI / 2.0));
    
    let parent = context.commands.spawn_bundle(PbrBundle {
        mesh: context.meshes.add(Mesh::from(shape::Plane { size: 0.1 })),
        material:  context.materials.add(node.color.into()),
        transform: transform,
        ..default()
    }).insert(SubstrateEntity{})
    .insert(RenderLayers::layer(RENDER_LAYER))
    .id();


    let child = context.commands.spawn_bundle(PbrBundle {
        mesh: context.meshes.add(Mesh::from(shape::Cube { size: node.size })),
        material:  context.materials.add(node.color.into()),
        transform: Transform::from_translation(Vec3{x: node.position.x, y: node.position.y, z: node.position.z}),
        ..default()
    }).insert(SubstrateEntity{}).insert(RenderLayers::layer(RENDER_LAYER)).id();

    context.commands.entity(parent).push_children(&[child]);
}


fn draw_text(_text: String, _size: f32, _transform: &Transform , _offset_x: f32, _offset_y: f32,_ctx: &mut DrawingContext){
    
    //return; //temporary, as there seems to be an asset loading bug
    #[allow(unreachable_code)] //temp disabled
    let mut text_transform = _transform.clone();
    let margin_as_fraction_of_text = 0.2;
    let margin = _size * margin_as_fraction_of_text;
  
    text_transform.translation.z += _offset_x;
    text_transform.translation.x += _offset_y;

    text_transform.translation.z -= _size / 72.;
    text_transform.translation.x -= margin / 72.;
    text_transform.translation.z -= margin / 72.;

    text_transform.rotate_local_x(PI / 2.0);
    text_transform.rotate_local_y(PI);
    // _ctx.commands.spawn_bundle(TextMeshBundle {
    //     text_mesh: TextMesh {
    //         text: _text,
    //         style: TextMeshStyle {
    //             color: Color::rgb(0., 0., 0.),
    //             font: _ctx.asset_server.load("fonts/FiraSans-Bold.ttf"),
    //             font_size: SizeUnit::NonStandard(_size),
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     },
    //     transform: text_transform,
    //     ..Default::default()
    // }).insert(RenderLayers::layer(RENDER_LAYER)).insert(SubstrateEntity{});
}
fn draw_connections(connections: &Vec<DiagramConnection>, ctx: &mut DrawingContext){
    
    let positive: Vec<&DiagramConnection> = connections.iter().filter(|c| c.weight > 0.1).collect();
    let neural: Vec<&DiagramConnection> = connections.iter().filter(|c| c.weight < 0.1 && c.weight > -0.1).collect();
    let negative: Vec<&DiagramConnection> = connections.iter().filter(|c| c.weight < 0.1).collect();

    ctx.commands.spawn_bundle(
        MaterialMeshBundle {
        mesh: ctx.meshes.add(Mesh::from(LineList {
            lines: positive.iter().map(|c|(Vec3{z: -c.from_position.y, x: c.from_position.x, y: c.from_position.z}, Vec3{z: -c.to_position.y, x: c.to_position.x, y: c.to_position.z})).collect(),
        })),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: ctx.line_materials.add(LineMaterial {
            color: Color::GREEN,
        }),

        ..default()
    }).insert(RenderLayers::layer(RENDER_LAYER)).insert(SubstrateEntity{});

    ctx.commands.spawn_bundle(
        MaterialMeshBundle {
        mesh: ctx.meshes.add(Mesh::from(LineList {
            lines: neural.iter().map(|c|(Vec3{z: -c.from_position.y, x: c.from_position.x, y: c.from_position.z}, Vec3{z: -c.to_position.y, x: c.to_position.x, y: c.to_position.z})).collect(),
        })),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: ctx.line_materials.add(LineMaterial {
            color: Color::BLACK
        }),
        ..default()
    }).insert(RenderLayers::layer(RENDER_LAYER)).insert(SubstrateEntity{});

    ctx.commands.spawn_bundle(
        MaterialMeshBundle {
        mesh: ctx.meshes.add(Mesh::from(LineList {
            lines: negative.iter().map(|c|(Vec3{z: -c.from_position.y, x: c.from_position.x, y: c.from_position.z}, Vec3{z: -c.to_position.y, x: c.to_position.x, y: c.to_position.z})).collect(),
        })),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: ctx.line_materials.add(LineMaterial {
            color: Color::RED
        }),
        ..default()
    }).insert(RenderLayers::layer(RENDER_LAYER)).insert(SubstrateEntity{});

}