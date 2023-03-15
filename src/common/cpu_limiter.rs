use core::time;

use perf_monitor::cpu::ProcessStat;

pub struct CpuLimiter;

impl CpuLimiter{
    pub fn limit(fraction: f32){
        let mut stat = ProcessStat::cur().unwrap();
        if stat.cpu().unwrap() > (fraction as f64){
            std::thread::sleep(time::Duration::from_millis(30));
        }
    }
}