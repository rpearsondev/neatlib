use std::f32::consts::PI;
use neatlib::common::NeatFloat;

use crate::objects::body_sensors::BodySensors;

pub struct Behaviours{
    pub has_fallen_over: bool,
    pub has_moved_quite_quick: bool,
    pub is_upright: bool,
    pub has_feet_in_alternate_positions: bool,
    pub both_legs_are_moving: bool,
    pub is_left_knee_high: bool,
    pub is_right_knee_high: bool,
    pub distance_from_upright: NeatFloat
}
impl Behaviours{
    pub fn from_sensors(sensors: BodySensors, previous_sensors: BodySensors) -> Self{
        let distance_from_upright = Behaviours::distance_from_upright(sensors); 
        Self{
            has_fallen_over: Behaviours::has_fallen_over(sensors, distance_from_upright),
            is_upright: distance_from_upright < 0.6,
            has_moved_quite_quick: Behaviours::has_moved_quite_quick(sensors, previous_sensors),
            has_feet_in_alternate_positions: Behaviours::has_feet_in_alternate_positions(sensors),
            both_legs_are_moving: Behaviours::both_legs_are_moving(sensors, previous_sensors),
            is_left_knee_high: Behaviours::is_left_knee_high(sensors),
            is_right_knee_high: Behaviours::is_right_knee_high(sensors),
            distance_from_upright:distance_from_upright ,
        }
    }
    fn has_moved_quite_quick(sensors: BodySensors, previous_sensors: BodySensors) -> bool{
        (sensors.distance_from_starting_point - previous_sensors.distance_from_starting_point).abs() > 0.2
    }
    fn distance_from_upright(sensors: BodySensors) -> NeatFloat{
        f32::max(f32::max(
            (sensors.torso_sensors.x_distance_in_radians_from_y_axis - (PI / 2.0)).abs(), 
            sensors.torso_sensors.y_distance_in_radians_from_y_axis.abs()), 
            (sensors.torso_sensors.z_distance_in_radians_from_y_axis - (PI / 2.0)).abs())
    }
    fn has_feet_in_alternate_positions(sensors: BodySensors) -> bool{
        (sensors.leg_right_lower_sensors.translation_y - sensors.leg_left_lower_sensors.translation_y).abs() > 1.0
    }
    fn has_fallen_over(sensors: BodySensors, distance_from_upright: NeatFloat) -> bool{
        sensors.torso_sensors.translation_y < 5.0 || distance_from_upright > 1.4
    }
    fn both_legs_are_moving(sensors: BodySensors, previous_sensors: BodySensors) -> bool{
        (sensors.left_hip_forward_joint.motor_position - previous_sensors.left_hip_forward_joint.motor_position).abs() > 0.2
        && (sensors.right_hip_forward_joint.motor_position - previous_sensors.right_hip_forward_joint.motor_position).abs() > 0.2
    }
    fn is_left_knee_high(sensors: BodySensors) -> bool{
        sensors.left_hip_forward_joint.motor_position > 1.05
    }
    fn is_right_knee_high(sensors: BodySensors) -> bool{
        sensors.right_hip_forward_joint.motor_position > 1.05
    }
}