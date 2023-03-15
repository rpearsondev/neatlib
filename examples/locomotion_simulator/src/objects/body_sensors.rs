use std::f32::consts::PI;

use bevy_rapier3d::{na::{UnitQuaternion, Vector3}};
use neatlib::common::NeatFloat;
use rapier3d::prelude::{RigidBodyHandle, MultibodyJointHandle};
use crate::simulator::rapier_context::RapierContext;
use super::body::Body;

#[derive(Copy, Clone)]
pub struct BodySensors{
    pub distance_from_starting_point: NeatFloat,
    pub torso_sensors: BodyPartSensors,
    pub left_hip_forward_joint: JointSensor,
    pub left_hip_outward_joint: JointSensor,
    pub leg_left_upper_sensors: BodyPartSensors,
    pub left_knee_joint: JointSensor,
    pub left_horizontal_leg_upper_sensors: BodyPartSensors,
    pub leg_left_lower_sensors: BodyPartSensors,
    pub right_hip_forward_joint: JointSensor,
    pub right_hip_outward_joint: JointSensor,
    pub leg_right_upper_sensors: BodyPartSensors,
    pub right_knee_joint: JointSensor,
    pub right_horizontal_leg_upper_sensors: BodyPartSensors,
    pub leg_right_lower_sensors: BodyPartSensors,
    pub counter_balance_z_joint: JointSensor,
    pub counter_balance_x_joint: JointSensor,
}

#[derive(Copy, Clone)]
pub struct BodyPartSensors{
    pub z_distance_in_radians_from_y_axis: NeatFloat,
    pub x_distance_in_radians_from_y_axis: NeatFloat,
    pub y_distance_in_radians_from_y_axis: NeatFloat,
    pub y_position_offset: NeatFloat,
    pub x_position_offset: NeatFloat,
    pub z_position_offset: NeatFloat,
    pub translation_z: f32,
    pub translation_x: f32,
    pub translation_y: f32,
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct JointSensor{
    pub offset: NeatFloat,
    pub motor_position: f32,
    pub max_limit: f32,
    pub min_limit: f32
}

impl BodySensors{
    pub fn from_body(context: &mut RapierContext, body: &Body) -> Option<Self> {

        if body.torso_handle.is_none(){
            return None;
        }

        let torso_handle = body.torso_handle.unwrap();
        let torso_position = Body::get_position(context, torso_handle);
        let mut distanct_from_starting_point = f32::sqrt(torso_position.translation.x * torso_position.translation.z);
        if distanct_from_starting_point.is_nan(){
            distanct_from_starting_point = 0.0;
        }
        Some(Self { 
            distance_from_starting_point: distanct_from_starting_point, 
            torso_sensors: BodyPartSensors::from_body_part(context, body.torso_handle.unwrap(), torso_handle), 
            left_horizontal_leg_upper_sensors: BodyPartSensors::from_body_part(context, body.left_horizontal_leg_upper.unwrap(), torso_handle),
            right_horizontal_leg_upper_sensors: BodyPartSensors::from_body_part(context, body.right_horizontal_leg_upper.unwrap(), torso_handle), 
            leg_left_upper_sensors: BodyPartSensors::from_body_part(context, body.left_leg_upper.unwrap(), torso_handle),
            leg_left_lower_sensors: BodyPartSensors::from_body_part(context, body.left_leg_lower.unwrap(), torso_handle), 
            leg_right_upper_sensors: BodyPartSensors::from_body_part(context, body.right_leg_upper.unwrap(), torso_handle), 
            leg_right_lower_sensors: BodyPartSensors::from_body_part(context, body.right_leg_lower.unwrap(), torso_handle),
            left_hip_forward_joint: JointSensor::from_joint(body.target_motor_positions.left_hip_forward_axis_position, context, body.left_hip_forward_joint.unwrap()),
            left_hip_outward_joint: JointSensor::from_joint(body.target_motor_positions.left_hip_outward_axis_position, context, body.left_hip_outward_joint.unwrap()),
            left_knee_joint: JointSensor::from_joint(body.target_motor_positions.left_knee_position, context, body.left_knee_joint.unwrap()),
            right_hip_forward_joint: JointSensor::from_joint(body.target_motor_positions.right_hip_forward_axis_position, context, body.right_hip_forward_joint.unwrap()),
            right_hip_outward_joint: JointSensor::from_joint(body.target_motor_positions.right_hip_outward_axis_position, context, body.right_hip_outward_joint.unwrap()),
            right_knee_joint: JointSensor::from_joint(body.target_motor_positions.right_knee_position, context, body.right_knee_joint.unwrap()), 
            counter_balance_z_joint: JointSensor::from_joint(body.target_motor_positions.counter_balance_z_position, context, body.counter_balance_z_joint.unwrap()),
            counter_balance_x_joint: JointSensor::from_joint(body.target_motor_positions.counter_balance_x_position, context, body.counter_balance_x_joint.unwrap()),
        })
    }

}

impl BodyPartSensors{
    pub fn from_body_part(context: &mut RapierContext, handle: RigidBodyHandle, torso_handle: RigidBodyHandle) -> Self{
        let torso_position = Body::get_position(context, torso_handle);
        let body = context.rigid_body_set.get(handle).unwrap();
        
        let quater_turn: f32 = PI / 2.0;
        BodyPartSensors{
            z_distance_in_radians_from_y_axis: body.position().rotation.angle_to(&UnitQuaternion::from_axis_angle(&Vector3::z_axis(), -quater_turn)),
            x_distance_in_radians_from_y_axis: body.position().rotation.angle_to(&UnitQuaternion::from_axis_angle(&Vector3::x_axis(), -quater_turn)),
            y_distance_in_radians_from_y_axis: body.position().rotation.angle_to(&UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.0)),
            y_position_offset: torso_position.translation.y - body.translation().y,
            x_position_offset: torso_position.translation.x - body.translation().x,
            z_position_offset: torso_position.translation.z - body.translation().z,
            translation_x: body.position().translation.x,
            translation_y: body.position().translation.y,
            translation_z: body.position().translation.z,
        }
    }
}

impl JointSensor{
    pub fn from_joint(motor_position: f32, context: &mut RapierContext, handle: MultibodyJointHandle) -> Self{
        let (multibody, index) = context.multibody_joint_set.get(handle).unwrap();
        let joint = multibody.link(index).unwrap();
        let angle = joint.local_to_parent().rotation.axis_angle();
        if angle.is_none(){
            return Self{
                max_limit: joint.joint.data.limits[0].max,
                min_limit: joint.joint.data.limits[0].min,
                offset: 0.0,
                motor_position: 0.0
            }
        }
        let (_, angle_radians) = angle.unwrap(); 
        Self{
            max_limit: joint.joint.data.limits[0].max,
            min_limit: joint.joint.data.limits[0].min,
            offset: angle_radians,
            motor_position: motor_position
        }
    }
}