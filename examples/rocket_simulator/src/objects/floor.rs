use bevy::{prelude::{PbrBundle, Mesh, Commands, ResMut, StandardMaterial, Assets, Color, Transform}, render::view::RenderLayers};
use rapier3d::prelude::*;
use crate::simulator::{rapier_context::{RapierContext}, object_engine_handle::ObjectEngineHandle, object_definition::ObjectDefinition};
use bevy::render::mesh::shape::Box;

#[derive(Clone)]
pub struct Floor;

impl Floor {
    pub fn new() -> Self{
        Self
    }
}

impl Floor{
    pub fn bevy_setup(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, standard_materials: &mut ResMut<Assets<StandardMaterial>>, render_layer: u8){
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(Box{min_x: -50.0, min_y: 0.0, min_z: -50.0, max_x:50.0, max_y: 1.0, max_z: 50.0})),
            material:  standard_materials.add(Color::PINK.into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        }).insert(RenderLayers::layer(render_layer));
    }
}

impl ObjectDefinition for Floor{
    fn create_in_engine(&mut self, context: &mut RapierContext) {
        let collider = ColliderBuilder::cuboid(100.0, 1.0, 1000.0).collision_groups(InteractionGroups::new(Group::GROUP_1, Group::ALL)).build();
        context.collider_set.insert(collider);        
    }

    fn get_engine_handle(&self) -> &Option<ObjectEngineHandle> {
        &None
    }
}