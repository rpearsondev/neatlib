use bevy::{prelude::{PbrBundle, Component, Mesh, Commands, ResMut, StandardMaterial, Assets, Color, Transform, Vec3}, render::{view::RenderLayers, mesh::{VertexAttributeValues, Indices}}};
use rapier3d::{prelude::*};
use crate::simulator::{rapier_context::{RapierContext}, object_engine_handle::ObjectEngineHandle, object_definition::ObjectDefinition};
#[derive(Clone)]
pub struct Rocket{
    handle: Option<ObjectEngineHandle>,
    starting_position: Vec3,
    pub collisions: u32
}
#[derive(Component)]
pub struct RocketComponent;

impl Rocket{
    pub fn new() -> Self{
        Self{
            handle: None,
            starting_position: Vec3{
                x: 0.0,
                y: 21.0,
                z: 0.0
            },
            collisions: 0
        }
    }
}

impl Rocket{
    pub fn get_position(&self, context: &mut RapierContext) -> Isometry<Real>{
        let body = context.rigid_body_set.get(self.handle.unwrap().rb_handle);
        return body.unwrap().position().clone();
    }
    pub fn get_speed(&self, context: &mut RapierContext) -> f32{
        let body = context.rigid_body_set.get(self.handle.unwrap().rb_handle);
        return body.unwrap().linvel().abs().magnitude().abs()
    }
    pub fn boost_right(&self, context: &mut RapierContext){
        let body = context.rigid_body_set.get_mut(self.handle.unwrap().rb_handle);
        let body_unwrapped = body.unwrap();
        let impulse = body_unwrapped.position().transform_vector(&vector![30.0, 20.0, 0.0]);
        let point = body_unwrapped.position().transform_point(&point![0.0, -20.0, 0.0]);
        body_unwrapped.apply_impulse_at_point(impulse, point, true);
    }
    pub fn boost_left(&self, context: &mut RapierContext){
        let body = context.rigid_body_set.get_mut(self.handle.unwrap().rb_handle);
        let body_unwrapped = body.unwrap();
        let impulse = body_unwrapped.position().transform_vector(&vector![-30.0, 20.0, 0.0]);
        let point = body_unwrapped.position().transform_point(&point![0.0, -20.0, 0.0]);
        body_unwrapped.apply_impulse_at_point(impulse, point, true);
    }
    pub fn boost_up(&self, context: &mut RapierContext){
        let body = context.rigid_body_set.get_mut(self.handle.unwrap().rb_handle);
        let body_unwrapped = body.unwrap();
        let impulse = body_unwrapped.position().transform_vector(&vector![0.0, 280.0, 0.0]);
        let point = body_unwrapped.position().transform_point(&point![0.0, -20.0, 0.0]);
        body_unwrapped.apply_impulse_at_point(impulse, point, true);
    }
    pub fn bevy_setup(&self, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, standard_materials: &mut ResMut<Assets<StandardMaterial>>, render_layer: u8){
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(bevy::render::mesh::shape::Capsule { radius: 2.5, rings: 5, depth:20.0, ..Default::default() })),
            material:  standard_materials.add(Color::RED.into()),
            transform: Transform::from_xyz(self.starting_position.x, self.starting_position.y, self.starting_position.z),
            ..Default::default()
        }).insert(RenderLayers::layer(render_layer)).insert(RocketComponent{});
    }
}

impl ObjectDefinition for Rocket{
    fn create_in_engine(&mut self, context: &mut RapierContext) {
        let rigid_body = RigidBodyBuilder::dynamic()
        .translation(vector![self.starting_position.x, self.starting_position.y, self.starting_position.z])
        .additional_mass_properties(MassProperties::new(
            point![0.0, 1.0, 0.0],
            0.5,
            vector![0.01, 0.01, 0.01],
        ))
        .build();

        let m = Mesh::from(bevy::render::mesh::shape::Capsule { radius: 2.5, rings: 5, depth:20.0, ..Default::default() });
        let collider = extract_mesh_vertices_indices(&m).unwrap();
        let collider = ColliderBuilder::trimesh(collider.0, collider.1)
        .restitution(2.0)
        .density(1.0)
        .collision_groups(InteractionGroups::new(Group::GROUP_1, Group::ALL)).build();        
        
        let handle = context.rigid_body_set.insert(rigid_body);
        context.collider_set.insert_with_parent(collider, handle, &mut context.rigid_body_set);
    
        self.handle = Some(ObjectEngineHandle {  
            rb_handle: handle
        });
    }

    fn get_engine_handle(&self) -> &Option<ObjectEngineHandle> {
        &self.handle
    }
}

fn extract_mesh_vertices_indices(mesh: &Mesh) -> Option<(Vec<rapier3d::na::Point3<Real>>, Vec<[u32; 3]>)> {
    let vertices = mesh.attribute(Mesh::ATTRIBUTE_POSITION)?;
    let indices = mesh.indices()?;

    let vtx: Vec<_> = match vertices {
        VertexAttributeValues::Float32(vtx) => Some(
            vtx.chunks(3)
                .map(|v| point![v[0] as Real, v[1] as Real, v[2] as Real])
                .collect(),
        ),
        VertexAttributeValues::Float32x3(vtx) => Some(
            vtx.iter()
                .map(|v| point![v[0] as Real, v[1] as Real, v[2] as Real])
                .collect(),
        ),
        _ => None,
    }?;

    let idx = match indices {
        Indices::U16(idx) => idx
            .chunks_exact(3)
            .map(|i| [i[0] as u32, i[1] as u32, i[2] as u32])
            .collect(),
        Indices::U32(idx) => idx.chunks_exact(3).map(|i| [i[0], i[1], i[2]]).collect(),
    };

    Some((vtx, idx))
}