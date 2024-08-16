#[derive(Clone)]
pub enum IterationsTemperature {
    ConstIterTemp { iterations: usize },
}
impl IterationsTemperature {
    pub fn const_iter_temp(iterations: usize) -> IterationsTemperature {
        IterationsTemperature::ConstIterTemp { iterations }
    }

    /// How many iterations for a given temperature
    pub(crate) fn get_iterations(&self, _temp: usize) -> usize {
        match &self {
            IterationsTemperature::ConstIterTemp { iterations } => *iterations,
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::simulated_annealing::IterationsTemperature;

    #[test]
    fn get_iterations_const() {
        let constant = IterationsTemperature::const_iter_temp(1000);
        assert_eq!(constant.get_iterations(5), 1000);
    }
}
