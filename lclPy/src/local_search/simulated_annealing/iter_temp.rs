#[derive(Clone)]
pub enum IterationsTemperature {
    ConstIterTemp { iterations: usize },
}
impl IterationsTemperature {
    pub(crate) fn get_iterations(&self, _temp: usize) -> usize {
        match &self {
            IterationsTemperature::ConstIterTemp { iterations } => {
                *iterations
            }
        }
    }
}
#[cfg(test)]
mod tests{
    use crate::local_search::simulated_annealing::IterationsTemperature::ConstIterTemp;

    #[test]
    fn get_iterations_const() {
        let constant = ConstIterTemp {iterations:1000};
        assert_eq!(constant.get_iterations(5), 1000);
    }
}
