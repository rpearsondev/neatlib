use crate::neat::{trainer::{configuration::Configuration, config_regulators::config_regulator::ConfigRegulator}, population::GenerationMember, genome::neat::NeatGenome};
use super::models::run_stats::RunStats;
pub enum FromHostEvent {
    BestNewGenome(GenerationMember<NeatGenome>),
    ConfigUpdate(Configuration),
    RegulatorUpdate(Vec<ConfigRegulator>),
    GenerationChange(u32),
    RunStats(RunStats),
    HitSuccessThreshold(u32),
    SetRunUntil(u32)
}
