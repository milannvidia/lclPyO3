#[derive(Clone)]
pub enum CoolingFunction {
    GeometricCooling { alpha: f64 },
}
impl CoolingFunction {
    pub fn geometric_cooling(alpha: f64) -> CoolingFunction {
        CoolingFunction::GeometricCooling { alpha }
    }
    /// Used to get the next temperature for simulated annealing
    pub(crate) fn get_next_temp(&self, temp: usize) -> usize {
        match &self {
            CoolingFunction::GeometricCooling { alpha } => {
                let result = alpha * temp as f64;
                result.round() as usize
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::simulated_annealing::CoolingFunction;
    #[test]
    fn get_next_temp_geometric() {
        let geo = CoolingFunction::geometric_cooling(0.5);
        assert_eq!(geo.get_next_temp(1000), 500usize);
    }
}
