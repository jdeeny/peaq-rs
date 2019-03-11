/// A Model Output Variable
pub trait Mov {
    fn name() -> &'static str;
}

mod bandwidth;
pub use bandwidth::Bandwidth;


/*



pub struct WindowedModulationDifference {

}

impl Mov for WindowedModulationDifference {
    fn name() -> &'static str { "WindowedModulationDifference" }

}

pub struct AverageBlockDistortion {

}

impl Mov for AverageBlockDistortion {
    fn name() -> &'static str { "AverageBlockDistortion" }
}

pub struct ErrorHarmonicStructure {

}

impl Mov for ErrorHarmonicStructure {
    fn name() -> &'static str { "ErrorHarmonicStructure" }

}

pub struct AverageModulationDistance {

}

impl Mov for AverageModulationDistance {
    fn name() -> &'static str { "AverageModulationDistance" }

}

pub struct DistortionLoudness {

}

impl Mov for DistortionLoudness {
    fn name() -> &'static str { "DistortionLoudness" }
}

pub struct MaximumFilteredDetectionProbability {

}

impl Mov for MaximumFilteredDetectionProbability {
    fn name() -> &'static str { "MaximumFilteredDetectionProbability" }
}

pub struct NoiseLoudnessAsym {

}

impl Mov for NoiseLoudnessAsym {
    fn name() -> &'static str { "NoiseLoudnessAsym" }
}

pub struct SegmentalNoiseMaskRatio {

}

impl Mov for SegmentalNoiseMaskRatio {
    fn name() -> &'static str { "SegmentalNoiseMaskRatio" }

}

pub struct LinearDistortion {

}

impl Mov for LinearDistortion {
    fn name() -> &'static str { "LinearDistortion" }
}

*/
