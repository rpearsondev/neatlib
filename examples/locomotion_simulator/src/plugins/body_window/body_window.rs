use std::{ops::RangeInclusive};

use bevy::prelude::{ResMut, Plugin, App};
use bevy_egui::{*, egui::{Ui, RichText, FontId, WidgetText, emath}};

use crate::{simulation_renderer::SimulationRun, objects::{body_sensors::{BodyPartSensors, JointSensor}, motor_positions::MotorPositions, body::{COUNTER_BALANCE_Z_LIMIT, LEG_HIP_FORWARD_ROTATION_LIMIT, KNEE_ROTATION_LIMIT}}};

pub struct BodyWindow{
}

pub struct BodyWindowResource{
    pub is_open: bool
}

impl Plugin for BodyWindow {
    fn build(&self, app: &mut App) {
        app.insert_resource(BodyWindowResource{
            is_open: true
        });
        app.insert_resource(MotorPositions{
            left_hip_forward_axis_position: 0.0,
            left_hip_outward_axis_position: 0.0,
            right_hip_forward_axis_position: 0.0,
            right_hip_outward_axis_position: 0.0,
            left_knee_position: 0.0,
            right_knee_position: 0.0,
            counter_balance_z_position: 0.0,
            counter_balance_x_position: 0.0
        });
        app.add_system(body_window);
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn body_window(
    mut egui_ctx: ResMut<EguiContext>,
    mut sql_resource: ResMut<BodyWindowResource>,
    mut current_run: ResMut<SimulationRun>,
    mut motor_positions: ResMut<MotorPositions>
) {
    let simulation_run = current_run.as_mut();
    let context = &mut simulation_run.simulation.rapier_context;
    let sensors = simulation_run.body.get_sensors(context);
    
    egui::Window::new("Body")
    .vscroll(true)
    .open(&mut sql_resource.is_open)
    .default_width(400.0)
    .default_height(400.0)
    .show(egui_ctx.ctx_mut(), |ui| {  
        if sensors.is_some(){
            let sensors_unwrapped = sensors.unwrap();

            ui.collapsing("Motors", |ui| {
                table(ui, |ui| {
                    add_slider_row(ui, "counter_balance_z_position", &mut motor_positions.counter_balance_z_position, -COUNTER_BALANCE_Z_LIMIT..=COUNTER_BALANCE_Z_LIMIT);
                    add_slider_row(ui, "left_hip_x_axis_position", &mut motor_positions.left_hip_forward_axis_position, -LEG_HIP_FORWARD_ROTATION_LIMIT..=LEG_HIP_FORWARD_ROTATION_LIMIT);
                    add_slider_row(ui, "left_hip_z_axis_position", &mut motor_positions.left_hip_outward_axis_position, -LEG_HIP_FORWARD_ROTATION_LIMIT..=LEG_HIP_FORWARD_ROTATION_LIMIT);
                    add_slider_row(ui, "right_hip_x_axis_position", &mut motor_positions.right_hip_forward_axis_position, -LEG_HIP_FORWARD_ROTATION_LIMIT..=LEG_HIP_FORWARD_ROTATION_LIMIT);
                    add_slider_row(ui, "right_hip_z_axis_position", &mut motor_positions.right_hip_outward_axis_position, -LEG_HIP_FORWARD_ROTATION_LIMIT..=LEG_HIP_FORWARD_ROTATION_LIMIT);
                    add_slider_row(ui, "left_knee_position", &mut motor_positions.left_knee_position, -KNEE_ROTATION_LIMIT..=0.0);
                    add_slider_row(ui, "right_knee_position", &mut motor_positions.right_knee_position, -KNEE_ROTATION_LIMIT..=0.0);
                });
            });

            ui.collapsing("Body", |ui| {
                table(ui, |ui| {
                    add_row(ui, "distance_from_starting_point", sensors_unwrapped.distance_from_starting_point);
                });
            });

            body_part(ui, "Left Leg Upper", sensors_unwrapped.leg_left_upper_sensors);
            body_part(ui, "Left Leg Lower", sensors_unwrapped.leg_left_lower_sensors);
            body_part(ui, "Right Leg Upper", sensors_unwrapped.leg_right_upper_sensors);
            body_part(ui, "Right Leg Lower", sensors_unwrapped.leg_right_lower_sensors);
            body_part(ui, "Torso", sensors_unwrapped.torso_sensors);
            joint(ui, "Left Hip Forward", sensors_unwrapped.left_hip_forward_joint);
            joint(ui, "Left Hip Outward", sensors_unwrapped.left_hip_outward_joint);
            joint(ui, "Left Knee", sensors_unwrapped.left_knee_joint);
            joint(ui, "Right Hip Forward", sensors_unwrapped.right_hip_forward_joint);
            joint(ui, "Right Hip Outward", sensors_unwrapped.right_hip_outward_joint);
            joint(ui, "Right Knee", sensors_unwrapped.right_knee_joint);
            joint(ui, "Counter Balance", sensors_unwrapped.counter_balance_z_joint);
        }
    });
}

pub fn body_part(ui: &mut Ui, section_label: &str, body_part_sensors: BodyPartSensors){
    ui.collapsing(section_label, |ui| {
        table(ui, |ui| {
            add_row(ui, "x_distance_in_radians_from_y_axis", body_part_sensors.x_distance_in_radians_from_y_axis);
            add_row(ui, "x_position_offset", body_part_sensors.x_position_offset);
            add_row(ui, "y_distance_in_radians_from_y_axis", body_part_sensors.y_distance_in_radians_from_y_axis);
            add_row(ui, "y_position_offset", body_part_sensors.y_position_offset);
            add_row(ui, "z_distance_in_radians_from_y_axis", body_part_sensors.z_distance_in_radians_from_y_axis);
            add_row(ui, "z_position_offset", body_part_sensors.z_position_offset);
            add_row(ui, "translation_x", body_part_sensors.translation_x);
            add_row(ui, "translation_y", body_part_sensors.translation_y);
            add_row(ui, "translation_z", body_part_sensors.translation_z);
        });
    });
}

pub fn joint(ui: &mut Ui, section_label: &str, joint: JointSensor){
    ui.collapsing(section_label, |ui| {
        table(ui, |ui| {
            add_row(ui, "motor_position", joint.motor_position);
            add_row(ui, "offset", joint.offset);
        });
    });
}

pub fn add_row(ui: &mut Ui, label: &str, value: f32){
    ui.label(RichText::new(label.to_string()).font(FontId::proportional(12.0)));
    ui.label(RichText::new(format!("{}", value )).font(FontId::proportional(12.0)));
    ui.end_row();
}

fn add_slider_row<Num: emath::Numeric>(ui: &mut Ui, label: impl Into<WidgetText>, value: &mut Num,  range: RangeInclusive<Num>){
    ui.label(label);
    ui.add(egui::Slider::new(value, range));
    ui.end_row();
}

fn table<R>(ui: &mut Ui, contents: impl FnOnce(&mut Ui) -> R){
    egui::Grid::new("")
    .num_columns(2)
    .spacing([40.0, 4.0])
    .striped(true)
    .show(ui, contents);
}
