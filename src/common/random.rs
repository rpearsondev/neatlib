use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use rand_distr::StandardNormal;

use super::NeatFloat;
pub struct Random;
impl Random{
    pub fn gen_range_f32( low_inc: NeatFloat, high_exc: NeatFloat) -> NeatFloat{
        let mut rng = SmallRng::from_entropy();
        rng.gen_range(low_inc..high_exc)
    }
    pub fn gen_range_usize( low_inc: usize, high_exc: usize) -> usize{
        let mut rng = SmallRng::from_entropy();
        rng.gen_range(low_inc..high_exc)
    }
    pub fn gen_range_i64( low_inc: i64, high_exc: i64) -> i64{
        let mut rng = SmallRng::from_entropy();
        rng.gen_range(low_inc..high_exc)
    }
    pub fn gen_bool(prob: NeatFloat) -> bool{
        let mut rng = SmallRng::from_entropy();
        rng.gen_bool(prob as f64)
    }
    pub fn standard_normal() -> NeatFloat{
        let mut rng = SmallRng::from_entropy();
        rng.sample(StandardNormal)
    }
}