use bevy::{prelude::{PbrBundle, Mesh, Commands, ResMut, StandardMaterial, Assets, Color, Transform, Vec3, Component}, render::{view::RenderLayers, mesh::{VertexAttributeValues, Indices}}};


use rapier3d::{prelude::*};
use crate::simulator::{rapier_context::{RapierContext}, object_engine_handle::ObjectEngineHandle, object_definition::ObjectDefinition};

#[derive(Clone)]
pub struct Target{
    handle: Option<ObjectEngineHandle>,
    pos: Vec3
}

#[derive(Component)]
pub struct TargetComponent;

impl Target{
    pub fn new() -> Self{
        let new_pos = Vec3{x: 15.0, y: 80.0, z: 0.0};
        Self{
            handle: None,
            pos: new_pos
        }
    }
}

impl Target{
    pub fn get_position(&self, context: &mut RapierContext) -> Isometry<f32>{
        let body = context.rigid_body_set.get(self.handle.unwrap().rb_handle);
        return body.unwrap().position().clone();
    }
    pub fn move_to_elsewhere(&self, context: &mut RapierContext){
        let body = context.rigid_body_set.get_mut(self.handle.unwrap().rb_handle);
        let unwrapped = body.unwrap();
        let mut x = unwrapped.translation().x;
        if x == -15.0 {
            x = 15.0;
        }else{
            x = -15.0;
        }
        let y = unwrapped.translation().y;
        unwrapped.set_translation(vector![x, y + 40.0, 0.0], true)
    }
    pub fn bevy_setup(&self, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, standard_materials: &mut ResMut<Assets<StandardMaterial>>, render_layer: u8){
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(bevy::render::mesh::shape::Cube {size:5.0 , ..Default::default() })),
            material:  standard_materials.add(Color::CYAN.into()),
            transform: Transform::from_xyz(self.pos.x, self.pos.y, self.pos.z),
            ..Default::default()
        }).insert(RenderLayers::layer(render_layer)).insert(TargetComponent);
    }
}

impl ObjectDefinition for Target{
    fn create_in_engine(&mut self, context: &mut RapierContext) {
        let rigid_body = RigidBodyBuilder::fixed()
        .translation(vector![self.pos.x,self.pos.y, self.pos.z])
        .build();

        let m = Mesh::from(bevy::render::mesh::shape::Cube {size:5.0 , ..Default::default() });
        let collider = extract_mesh_vertices_indices(&m).unwrap();
        let collider = ColliderBuilder::trimesh(collider.0, collider.1)
        .restitution(2.0)
        //.collision_groups(InteractionGroups::new(Group::GROUP_1, Group::NONE))
        .active_events(ActiveEvents::COLLISION_EVENTS)
        .build();        
        
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