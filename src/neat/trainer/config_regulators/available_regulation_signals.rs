use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::{neat::trainer::run_signals::run_signals::RunSignals, common::NeatFloat};
#[derive(Clone, Copy, Debug, EnumIter, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum AvailableRegulationSignals{
    NumberOfSpecies,
    AvgSpeciesFitnessImprovementInLast10GensComparedToLast100GensAsFactor,
    MaxSpeciesFitnessImprovementInLast10GensComparedToLast100GensAsFactor,
    Oscillator10Gen,
}

impl AvailableRegulationSignals{
    pub fn get_signal_value(&self, run_signals: &RunSignals) -> NeatFloat{
        match self {
            AvailableRegulationSignals::NumberOfSpecies => run_signals.number_of_species,
            AvailableRegulationSignals::AvgSpeciesFitnessImprovementInLast10GensComparedToLast100GensAsFactor => run_signals.avg_fitness_improvement_in_last10_gens_compared_to_last100_gens_as_factor,
            AvailableRegulationSignals::MaxSpeciesFitnessImprovementInLast10GensComparedToLast100GensAsFactor => run_signals.max_fitness_improvement_in_last10_gens_compared_to_last100_gens_as_factor,
            AvailableRegulationSignals::Oscillator10Gen => run_signals.oscillator_10_gen,
        }
    }
}