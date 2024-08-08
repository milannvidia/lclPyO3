#[derive(Clone)]
pub enum CoolingFunction {
    GeometricCooling { alpha: f64 },
}
impl CoolingFunction {
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
mod tests{
    use crate::local_search::simulated_annealing::CoolingFunction::GeometricCooling;
    #[test]
    fn get_next_temp_geometric() {
        let geo=GeometricCooling {alpha:0.5f64};
        assert_eq!(geo.get_next_temp(1000),500usize);
    }
}
