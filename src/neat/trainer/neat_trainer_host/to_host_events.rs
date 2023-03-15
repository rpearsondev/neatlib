use crate::neat::trainer::{configuration::Configuration, config_regulators::config_regulator::ConfigRegulator};
use super::models::generic_operation::GenericOperation;

pub enum ToHostEvents {
    UpdateConfig(Configuration),
    Stop(),
    SetRunUntil(Option<u32>),
    Reset(),
    RequestRunStats(),
    RequestLatestConfig(),
    UpdateConfigRegulators(Vec<ConfigRegulator>),
    GenericOperation(GenericOperation),
}
