pub type NeatFloat = f32;
pub struct NeatFloatExtensions;
impl NeatFloatExtensions{
    pub fn abs_diff(a: NeatFloat, b: NeatFloat) -> NeatFloat{
        (a - b).abs()
    }
}