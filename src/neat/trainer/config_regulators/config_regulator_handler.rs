use crate::neat::trainer::{configuration::Configuration, run_signals::run_signals::RunSignals};

use super::{config_regulator::ConfigRegulator, simple_regulator::SimpleRegulator};

pub struct ConfigRegulatorHandler;

impl ConfigRegulatorHandler{
    pub fn handle(regulator: &ConfigRegulator, config: &mut Configuration, run_signals: &RunSignals){
        SimpleRegulator::regulate(
            regulator.property_to_change.get_configuration_value(config),
            regulator.max_value_of_property,
            regulator.min_value_of_property,
            regulator.signal_name.get_signal_value(&run_signals),
            regulator.signal_target,
            regulator.when_signal_above_change_factor,
            regulator.when_signal_below_change_factor
        )
    }
}