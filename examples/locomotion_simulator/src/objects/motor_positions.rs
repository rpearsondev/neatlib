use neatlib::common::NeatFloat;

#[derive(Clone, Copy, Debug, Default)]
pub struct MotorPositions{
    pub left_hip_forward_axis_position: NeatFloat,
    pub left_hip_outward_axis_position: NeatFloat,
    pub right_hip_forward_axis_position: NeatFloat,
    pub right_hip_outward_axis_position: NeatFloat,
    pub left_knee_position: NeatFloat,
    pub right_knee_position: NeatFloat,
    pub counter_balance_z_position: NeatFloat,
    pub counter_balance_x_position: NeatFloat
}