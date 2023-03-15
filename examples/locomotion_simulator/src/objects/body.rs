use bevy::{prelude::{PbrBundle, Component, Mesh, Commands, ResMut, StandardMaterial, Assets, Color, Transform, Vec3, Query}, render::{view::RenderLayers, mesh::{VertexAttributeValues, Indices}}};
use bevy_rapier3d::utils;
use rapier3d::{prelude::*};
use crate::{simulator::{rapier_context::{RapierContext}, object_definition::ObjectDefinition}, simulation_renderer::SimulationRun};

use super::{body_sensors::BodySensors, motor_positions::MotorPositions};

pub static LEG_UPPER_LENGTH :f32 = 3.0;
pub static LEG_WIDTH: f32 = 0.9;
pub static LEG_LOWER_WIDTH: f32 = 0.9;
pub static LEG_HIP_FORWARD_ROTATION_LIMIT: f32 = 1.3;
pub static LEG_HIP_OUTWARD_ROTATION_LIMIT: f32 = 0.4;
pub static LEG_LOWER_LENGTH: f32 =2.0;
pub static COUNTER_BALANCE_Z_HEIGHT:f32 = 1.5;
pub static COUNTER_BALANCE_Z_WIDTH:f32 = 1.0;
pub static COUNTER_BALANCE_Z_LIMIT:f32 = 1.0;
pub static COUNTER_BALANCE_X_HEIGHT:f32 = 2.1;
pub static COUNTER_BALANCE_X_WIDTH:f32 = 1.0;
pub static COUNTER_BALANCE_X_LIMIT:f32 = 1.4;
pub static KNEE_ROTATION_LIMIT: f32 = 1.5;
pub static TORSO_WIDTH:f32 = 1.3;
pub static RESTITUTION: f32 = 0.0001;

#[derive(Clone, Debug)]
pub struct Body{
    pub torso_handle: Option<RigidBodyHandle>,
    pub left_leg_upper: Option<RigidBodyHandle>,
    pub left_horizontal_leg_upper: Option<RigidBodyHandle>,
    pub right_leg_upper: Option<RigidBodyHandle>,
    pub right_horizontal_leg_upper: Option<RigidBodyHandle>,
    pub left_leg_lower: Option<RigidBodyHandle>,
    pub right_leg_lower: Option<RigidBodyHandle>,
    pub counter_balance_z: Option<RigidBodyHandle>,
    pub counter_balance_x: Option<RigidBodyHandle>,
    pub left_hip_forward_joint: Option<MultibodyJointHandle>,
    pub right_hip_forward_joint: Option<MultibodyJointHandle>,
    pub left_hip_outward_joint: Option<MultibodyJointHandle>,
    pub right_hip_outward_joint: Option<MultibodyJointHandle>,
    pub left_knee_joint: Option<MultibodyJointHandle>,
    pub right_knee_joint: Option<MultibodyJointHandle>,
    pub counter_balance_z_joint: Option<MultibodyJointHandle>,
    pub counter_balance_x_joint: Option<MultibodyJointHandle>,
    pub torso_position: Vec3,
    pub collisions: u32,
    pub torso_mesh: fn() -> Mesh,
    pub upper_leg_mesh: fn() -> Mesh,
    pub upper_leg_horizontal_mesh: fn() -> Mesh,
    pub lower_leg_mesh: fn() -> Mesh,
    pub counter_balance_z_mesh: fn() -> Mesh,
    pub counter_balance_x_mesh: fn() -> Mesh,
    pub target_motor_positions: MotorPositions
}


#[derive(Component)]
pub struct BodyPartUpdateComponent{
    handle: RigidBodyHandle
}

#[derive(Component)]
pub struct BodyTorsoComponent;

impl Body{
    pub fn new() -> Self{
        Self{
            torso_handle: None,
            left_leg_upper: None,
            left_horizontal_leg_upper: None,
            right_leg_upper: None,
            right_horizontal_leg_upper: None,
            left_leg_lower: None,
            right_leg_lower: None,
            counter_balance_z: None,
            counter_balance_x: None,
            left_hip_forward_joint: None,
            right_hip_outward_joint: None,
            left_hip_outward_joint: None,
            right_hip_forward_joint: None,
            left_knee_joint: None,
            right_knee_joint: None,
            counter_balance_z_joint: None,
            counter_balance_x_joint: None,
            torso_position: Vec3{
                x: 0.0,
                y: 7.0,
                z: 0.0
            },
            collisions: 0,
            torso_mesh: || Mesh::from(bevy::render::mesh::shape::Cube{size: TORSO_WIDTH}),
            upper_leg_horizontal_mesh: || {Mesh::from(bevy::render::mesh::shape::Box{ max_x: (LEG_WIDTH/2.0), min_x: -(LEG_WIDTH/2.0), max_y: LEG_WIDTH/2.0, min_y: -LEG_WIDTH/2.0, max_z: (LEG_WIDTH/2.0), min_z: -(LEG_WIDTH/2.0) })},
            upper_leg_mesh: || {Mesh::from(bevy::render::mesh::shape::Box{ max_x: (LEG_WIDTH/2.0), min_x: -(LEG_WIDTH/2.0), max_y: LEG_UPPER_LENGTH/2.0, min_y: -LEG_UPPER_LENGTH/2.0, max_z: (LEG_WIDTH/2.0), min_z: -(LEG_WIDTH/2.0) })},
            lower_leg_mesh: || {Mesh::from(bevy::render::mesh::shape::Box{ max_x: (LEG_LOWER_WIDTH/2.0), min_x: -(LEG_LOWER_WIDTH/2.0), max_y: LEG_LOWER_LENGTH/2.0, min_y: -LEG_LOWER_LENGTH/2.0, max_z: (LEG_LOWER_WIDTH/2.0), min_z: -(LEG_LOWER_WIDTH/2.0) })},
            counter_balance_z_mesh: || {Mesh::from(bevy::render::mesh::shape::Box{ max_x: (COUNTER_BALANCE_Z_WIDTH/2.0), min_x: (-COUNTER_BALANCE_Z_WIDTH/2.0), max_y: COUNTER_BALANCE_Z_HEIGHT / 2.0, min_y: -COUNTER_BALANCE_Z_HEIGHT / 2.0, max_z: (COUNTER_BALANCE_Z_WIDTH/2.0), min_z: (-COUNTER_BALANCE_Z_WIDTH/2.0) })},
            counter_balance_x_mesh: || {Mesh::from(bevy::render::mesh::shape::Box{ max_x: (COUNTER_BALANCE_X_WIDTH/2.0), min_x: (-COUNTER_BALANCE_X_WIDTH/2.0), max_y: COUNTER_BALANCE_X_HEIGHT / 2.0, min_y: -COUNTER_BALANCE_X_HEIGHT / 2.0, max_z: (COUNTER_BALANCE_X_WIDTH/2.0), min_z: (-COUNTER_BALANCE_X_WIDTH/2.0) })},
            target_motor_positions: MotorPositions { ..Default::default() }
        }
    }
}

impl Body{
  
    fn bevy_create(&self, handle:RigidBodyHandle, mesh: fn() -> Mesh, color: Color,commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, standard_materials: &mut ResMut<Assets<StandardMaterial>>, render_layer: u8){
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(mesh()),
            material:  standard_materials.add(color.into()),
            transform: Transform::from_xyz(self.torso_position.x, self.torso_position.y, self.torso_position.z),
            ..Default::default()
        }).insert(RenderLayers::layer(render_layer))
        .insert(BodyTorsoComponent)
        .insert(BodyPartUpdateComponent{
            handle: handle
        });
    }
    fn rapier_create(context: &mut RapierContext, mesh: fn() -> Mesh, translation: Vector<Real>) -> RigidBodyHandle{
        let rigid_body = RigidBodyBuilder::dynamic()
        .translation(translation)
        .additional_mass_properties(MassProperties::new(
            point![0.0, 1.0, 0.0],
            0.5,
            vector![0.0, 0.0, 0.0],
        ))
        .linvel(vector![0.0, 0.0, 0.0])
        .build();

        let mesh = mesh();
        let collider = extract_mesh_vertices_indices(&mesh).unwrap();
        let collider = ColliderBuilder::trimesh(collider.0, collider.1)
        .restitution(RESTITUTION)
        .density(1.0)
        .friction(4.0)
        .collision_groups(InteractionGroups::new(Group::GROUP_1, !Group::GROUP_1)).build();
        
        let handle = context.rigid_body_set.insert(rigid_body);
        context.collider_set.insert_with_parent(collider, handle, &mut context.rigid_body_set);
        handle
    }

    fn torso_bevy_create(&self, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, standard_materials: &mut ResMut<Assets<StandardMaterial>>, render_layer: u8){
        self.bevy_create(
            self.torso_handle.unwrap(),
            self.torso_mesh,
            Color::RED,
            commands,
            meshes, 
            standard_materials,
            render_layer
        );
    }
    fn torso_rapier_create(&mut self, context: &mut RapierContext) -> RigidBodyHandle {
        Self::rapier_create(context, self.torso_mesh, vector![self.torso_position.x, self.torso_position.y, self.torso_position.z])
    }

    fn left_leg_horizontal_upper_rapier_create(&mut self, context: &mut RapierContext) -> RigidBodyHandle {
        Self::rapier_create(context, self.upper_leg_horizontal_mesh, vector![0.0, 0.0, 0.0])
    }
    fn left_leg_horizontal_upper_bevy_create(&self, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, standard_materials: &mut ResMut<Assets<StandardMaterial>>, render_layer: u8){
        self.bevy_create(
            self.left_horizontal_leg_upper.unwrap(),
            self.upper_leg_horizontal_mesh,
            Color::RED,
            commands,
            meshes, 
            standard_materials,
            render_layer
        );
    }
    
    fn left_leg_upper_rapier_create(&mut self, context: &mut RapierContext) -> RigidBodyHandle {
        Self::rapier_create(context, self.upper_leg_mesh, vector![self.torso_position.x -(LEG_WIDTH/2.0) -(TORSO_WIDTH/2.0), self.torso_position.y - (LEG_UPPER_LENGTH /2.0), self.torso_position.z])
    }
    fn left_leg_upper_bevy_create(&self, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, standard_materials: &mut ResMut<Assets<StandardMaterial>>, render_layer: u8){
        self.bevy_create(
            self.left_leg_upper.unwrap(),
            self.upper_leg_mesh,
            Color::RED,
            commands,
            meshes, 
            standard_materials,
            render_layer
        );
    }

    fn right_leg_horizontal_upper_rapier_create(&mut self, context: &mut RapierContext) -> RigidBodyHandle {
        Self::rapier_create(context, self.upper_leg_horizontal_mesh, vector![0.0, 0.0, 0.0])
    }
    fn right_leg_horizontal_upper_bevy_create(&self, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, standard_materials: &mut ResMut<Assets<StandardMaterial>>, render_layer: u8){
        self.bevy_create(
            self.right_horizontal_leg_upper.unwrap(),
            self.upper_leg_horizontal_mesh,
            Color::RED,
            commands,
            meshes, 
            standard_materials,
            render_layer
        );
    }

    fn right_leg_upper_rapier_create(&mut self, context: &mut RapierContext) -> RigidBodyHandle {
        Self::rapier_create(context, self.upper_leg_mesh, vector![self.torso_position.x + (LEG_WIDTH/2.0) + (TORSO_WIDTH/2.0), self.torso_position.y - (LEG_UPPER_LENGTH /2.0), self.torso_position.z])
    }
    fn right_leg_upper_bevy_create(&self, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, standard_materials: &mut ResMut<Assets<StandardMaterial>>, render_layer: u8){
        self.bevy_create(
            self.right_leg_upper.unwrap(),
            self.upper_leg_mesh,
            Color::RED,
            commands,
            meshes, 
            standard_materials,
            render_layer
        );
    }

    fn left_leg_lower_rapier_create(&mut self, context: &mut RapierContext) -> RigidBodyHandle {
        Self::rapier_create(context, self.upper_leg_mesh, vector![self.torso_position.x + (LEG_WIDTH/2.0) + (TORSO_WIDTH/2.0), self.torso_position.y - (LEG_UPPER_LENGTH /2.0), self.torso_position.z])
    }
    fn left_leg_lower_bevy_create(&self, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, standard_materials: &mut ResMut<Assets<StandardMaterial>>, render_layer: u8){
        self.bevy_create(
            self.left_leg_lower.unwrap(),
            self.lower_leg_mesh,
            Color::RED,
            commands,
            meshes, 
            standard_materials,
            render_layer
        );
    }

    fn right_leg_lower_rapier_create(&mut self, context: &mut RapierContext) -> RigidBodyHandle {
        Self::rapier_create(context, self.upper_leg_mesh, vector![self.torso_position.x +(LEG_WIDTH/2.0) +(TORSO_WIDTH/2.0), self.torso_position.y - (LEG_LOWER_LENGTH /2.0) - LEG_UPPER_LENGTH, self.torso_position.z])
    }
    fn right_leg_lower_bevy_create(&self, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, standard_materials: &mut ResMut<Assets<StandardMaterial>>, render_layer: u8){
        self.bevy_create(
            self.right_leg_lower.unwrap(),
            self.lower_leg_mesh,
            Color::RED,
            commands,
            meshes, 
            standard_materials,
            render_layer
        );
    }

    fn top_counter_balance_z_rapier_create(&mut self, context: &mut RapierContext) -> RigidBodyHandle {
        Self::rapier_create(context, self.counter_balance_z_mesh, vector![self.torso_position.x, self.torso_position.y + COUNTER_BALANCE_Z_HEIGHT / 2.0, self.torso_position.z])
    }
    fn top_counter_balance_z_bevy_create(&self, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, standard_materials: &mut ResMut<Assets<StandardMaterial>>, render_layer: u8){
        self.bevy_create(
            self.counter_balance_z.unwrap(),
            self.counter_balance_z_mesh,
            Color::RED,
            commands,
            meshes, 
            standard_materials,
            render_layer
        );
    }

    fn top_counter_balance_x_rapier_create(&mut self, context: &mut RapierContext) -> RigidBodyHandle {
        Self::rapier_create(context, self.counter_balance_z_mesh, vector![self.torso_position.x, self.torso_position.y + COUNTER_BALANCE_Z_HEIGHT / 2.0, self.torso_position.z])
    }
    fn top_counter_balance_x_bevy_create(&self, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, standard_materials: &mut ResMut<Assets<StandardMaterial>>, render_layer: u8){
        self.bevy_create(
            self.counter_balance_x.unwrap(),
            self.counter_balance_z_mesh,
            Color::RED,
            commands,
            meshes, 
            standard_materials,
            render_layer
        );
    }

    pub fn joints_create(&mut self, context: &mut RapierContext){
        let forward_hip_axis = Vector::x_axis();
        let outward_hip_axis = Vector::z_axis();

        //left horizonal hip
        let joint = RevoluteJointBuilder::new(outward_hip_axis)
        .local_anchor1(point![-(TORSO_WIDTH/2.0)+1.0, 0.0, 0.0])
        .local_anchor2(point![(LEG_WIDTH/2.0), LEG_WIDTH/2.0, 0.0 ])
        .limits([-LEG_HIP_OUTWARD_ROTATION_LIMIT, 0.0]);
        self.left_hip_outward_joint = context.multibody_joint_set.insert(self.torso_handle.unwrap(), self.left_horizontal_leg_upper.unwrap(), joint, true);

        //right horizonal hip
        let joint = RevoluteJointBuilder::new(outward_hip_axis)
        .local_anchor1(point![(TORSO_WIDTH/2.0)-1.0, 0.0, 0.0])
        .local_anchor2(point![-(LEG_WIDTH/2.0), LEG_WIDTH/2.0, 0.0])
        .limits([0.0, LEG_HIP_FORWARD_ROTATION_LIMIT]);
        self.right_hip_outward_joint = context.multibody_joint_set.insert(self.torso_handle.unwrap(), self.right_horizontal_leg_upper.unwrap(), joint, true);
        
        //left forward hip
        let joint = RevoluteJointBuilder::new(forward_hip_axis)
        .local_anchor1(point![-(LEG_WIDTH/2.0), 0.0, 0.0])
        .local_anchor2(point![(LEG_WIDTH/2.0), LEG_UPPER_LENGTH/2.0, 0.0 ])
        .limits([-LEG_HIP_FORWARD_ROTATION_LIMIT, LEG_HIP_FORWARD_ROTATION_LIMIT]);
        let added = context.multibody_joint_set.insert(self.left_horizontal_leg_upper.unwrap(), self.left_leg_upper.unwrap(), joint, true);
        self.left_hip_forward_joint = added;


        //right forward hip
        let joint = RevoluteJointBuilder::new(forward_hip_axis)
        .local_anchor1(point![(LEG_WIDTH/2.0), 0.0, 0.0])
        .local_anchor2(point![-(LEG_WIDTH/2.0), LEG_UPPER_LENGTH/2.0, 0.0])
        .limits([-LEG_HIP_FORWARD_ROTATION_LIMIT, LEG_HIP_FORWARD_ROTATION_LIMIT]);
        
        self.right_hip_forward_joint = context.multibody_joint_set.insert(self.right_horizontal_leg_upper.unwrap(), self.right_leg_upper.unwrap(), joint, true);
        let knee_axis = Vector::x_axis();

        //left knee
        let joint = RevoluteJointBuilder::new(knee_axis)
        .local_anchor1(point![0.0, -LEG_UPPER_LENGTH / 2.0, 0.0])
        .local_anchor2(point![0.0, LEG_LOWER_LENGTH / 2.0, 0.0 ])
        .limits([-KNEE_ROTATION_LIMIT, KNEE_ROTATION_LIMIT]);
        self.left_knee_joint = context.multibody_joint_set.insert(self.left_leg_upper.unwrap(), self.left_leg_lower.unwrap(), joint, true);

        //right knee
        let joint = RevoluteJointBuilder::new(knee_axis)
        .local_anchor1(point![0.0, -LEG_UPPER_LENGTH / 2.0, 0.0])
        .local_anchor2(point![0.0, LEG_LOWER_LENGTH / 2.0, 0.0 ])
        .limits([-KNEE_ROTATION_LIMIT, KNEE_ROTATION_LIMIT]);
        self.right_knee_joint = context.multibody_joint_set.insert(self.right_leg_upper.unwrap(), self.right_leg_lower.unwrap(), joint, true);

        //counter balance z
        let joint = RevoluteJointBuilder::new(Vector::z_axis())
        .local_anchor1(point![0.0, TORSO_WIDTH / 2.0, 0.0])
        .local_anchor2(point![0.0, -COUNTER_BALANCE_Z_HEIGHT / 2.0, 0.0 ])
        .limits([-COUNTER_BALANCE_Z_LIMIT, COUNTER_BALANCE_Z_LIMIT]);
        self.counter_balance_z_joint = context.multibody_joint_set.insert(self.torso_handle.unwrap(), self.counter_balance_z.unwrap(), joint, true);

        //counter balance x
        let joint = RevoluteJointBuilder::new(Vector::x_axis())
        .local_anchor1(point![0.0, COUNTER_BALANCE_Z_HEIGHT / 2.0, 0.0])
        .local_anchor2(point![0.0, -COUNTER_BALANCE_X_HEIGHT / 2.0, 0.0 ])
        .limits([-COUNTER_BALANCE_X_LIMIT, COUNTER_BALANCE_X_LIMIT]);
        self.counter_balance_x_joint = context.multibody_joint_set.insert(self.counter_balance_z.unwrap(), self.counter_balance_x.unwrap(), joint, true);


    }

    //all body parts
    pub fn bevy_setup(&self, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, standard_materials: &mut ResMut<Assets<StandardMaterial>>, render_layer: u8){
        self.torso_bevy_create(commands, meshes, standard_materials, render_layer);
        self.left_leg_upper_bevy_create(commands, meshes, standard_materials, render_layer);
        self.left_leg_horizontal_upper_bevy_create(commands, meshes, standard_materials, render_layer);
        self.right_leg_upper_bevy_create(commands, meshes, standard_materials, render_layer);
        self.right_leg_horizontal_upper_bevy_create(commands, meshes, standard_materials, render_layer);
        self.left_leg_lower_bevy_create(commands, meshes, standard_materials, render_layer);
        self.right_leg_lower_bevy_create(commands, meshes, standard_materials, render_layer);
        self.top_counter_balance_z_bevy_create(commands, meshes, standard_materials, render_layer);
        self.top_counter_balance_x_bevy_create(commands, meshes, standard_materials, render_layer);
    }
    pub fn torso_get_speed(&self, context: &mut RapierContext) -> f32{
        let body = context.rigid_body_set.get(self.torso_handle.unwrap());
        return body.unwrap().linvel().abs().magnitude().abs()
    }
    pub fn set_motor_positions(&mut self, context: &mut RapierContext, motor_positions: MotorPositions){
        let body = context.rigid_body_set.get_mut(self.torso_handle.unwrap());
        body.unwrap().wake_up(true);
        self.target_motor_positions = motor_positions;
        Self::set_revolute_motor_position(context, self.counter_balance_z_joint.unwrap(), motor_positions.counter_balance_z_position);
        Self::set_revolute_motor_position(context, self.left_hip_forward_joint.unwrap(), motor_positions.left_hip_forward_axis_position);
        Self::set_revolute_motor_position(context, self.right_hip_forward_joint.unwrap(), motor_positions.right_hip_forward_axis_position);
        Self::set_revolute_motor_position(context, self.left_hip_outward_joint.unwrap(), motor_positions.left_hip_outward_axis_position);
        Self::set_revolute_motor_position(context, self.right_hip_outward_joint.unwrap(), motor_positions.right_hip_outward_axis_position);
        Self::set_revolute_motor_position(context, self.left_knee_joint.unwrap(), motor_positions.left_knee_position);
        Self::set_revolute_motor_position(context, self.right_knee_joint.unwrap(), motor_positions.right_knee_position);
    }
    fn set_revolute_motor_position(context: &mut RapierContext, handle: MultibodyJointHandle, motor_position: f32){
        let (multibody, index) = context.multibody_joint_set.get_mut(handle).unwrap();
        let joint = multibody.link_mut(index).unwrap();
        let revolute_joint = joint.joint.data.as_revolute_mut().unwrap();
        revolute_joint.set_motor_position(motor_position, 0.5, 0.1);
    }

    pub fn get_position(context: &mut RapierContext, handle: RigidBodyHandle) -> Isometry<Real>{
        let body = context.rigid_body_set.get(handle);
        return body.unwrap().position().clone();
    }
    pub fn get_sensors(&self, context: &mut RapierContext) -> Option<BodySensors>{
        BodySensors::from_body(context, self)
    }
    pub fn bevy_update( context: &mut SimulationRun, body_query: &mut Query<(&mut Transform, &BodyPartUpdateComponent)>){
        for (mut transform, component) in body_query.iter_mut(){
            let ball_position = Self::get_position(&mut context.simulation.rapier_context, component.handle);
            *transform = utils::iso_to_transform(&ball_position, 1.0)
        }
    }
}

impl ObjectDefinition for Body{
    fn create_in_engine(&mut self, context: &mut RapierContext) {
        self.torso_handle = Some(self.torso_rapier_create(context));
        self.left_leg_upper = Some(self.left_leg_upper_rapier_create(context));
        self.left_horizontal_leg_upper = Some(self.left_leg_horizontal_upper_rapier_create(context));
        self.right_leg_upper = Some(self.right_leg_upper_rapier_create(context));
        self.right_horizontal_leg_upper = Some(self.right_leg_horizontal_upper_rapier_create(context));
        self.left_leg_lower = Some(self.left_leg_lower_rapier_create(context));
        self.right_leg_lower = Some(self.right_leg_lower_rapier_create(context));
        self.counter_balance_z = Some(self.top_counter_balance_z_rapier_create(context));
        self.counter_balance_x = Some(self.top_counter_balance_x_rapier_create(context));
        self.joints_create(context);
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