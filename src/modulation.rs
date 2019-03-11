pub struct ModulationProcessor {

}

impl ModulationProcessor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn process(&self, input_data: &[f64]) {

    }

    pub fn modulation(&self) -> Vec<f64> {
        vec!(0.0, 0.0)
    }

    pub fn avg_loudness(&self) -> Vec<f64> {
        vec!(0.0, 0.0)
    }
}
