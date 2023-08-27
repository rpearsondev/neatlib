
use serde::{Deserialize, Serialize};
use crate::{common::NeatFloat};
use super::{regulatable_configuration_properties::RegulatableConfigurationProperties, available_regulation_signals::AvailableRegulationSignals};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct ConfigRegulator{
    pub start_generation: u32,
    pub property_to_change: RegulatableConfigurationProperties,
    pub max_value_of_property: NeatFloat,
    pub min_value_of_property: NeatFloat,
    pub signal_name: AvailableRegulationSignals,
    pub signal_target: NeatFloat,
    pub when_signal_above_change_factor: NeatFloat,
    pub when_signal_below_change_factor: NeatFloat
}