use crate::{common::NeatFloat};

pub struct SimpleRegulator;

impl SimpleRegulator{
    pub fn regulate(configuration_value: &mut NeatFloat, max: NeatFloat, min: NeatFloat, signal: NeatFloat, signal_target: NeatFloat, when_above_factor: NeatFloat, when_below_factor:NeatFloat){
        if signal < signal_target {
            *configuration_value += *configuration_value * when_below_factor;
        }else  if signal > signal_target {
            *configuration_value += *configuration_value * when_above_factor;
        }
        if *configuration_value > max{
            *configuration_value = max;
        }
        if *configuration_value < min {
            *configuration_value = min;
        }
    }
}