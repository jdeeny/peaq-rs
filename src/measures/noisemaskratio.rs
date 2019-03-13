use crate::measures::Mov;

pub struct NoiseMaskRatio {

}

impl Mov for NoiseMaskRatio {
    fn name() -> &'static str { "NoiseMaskRatio" }
}
