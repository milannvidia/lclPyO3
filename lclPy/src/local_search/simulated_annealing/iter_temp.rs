#[derive(Clone)]
pub enum IterationsTemperature {
    CnstIterTemp { iterations: usize },
}
impl IterationsTemperature {
    pub(crate) fn get_iterations(&self, _temp: usize) -> usize {
        match &self {
            IterationsTemperature::CnstIterTemp { iterations } => {
                *iterations
            }
        }
    }
}
#[cfg(test)]
mod tests{
    use crate::local_search::simulated_annealing::IterationsTemperature::CnstIterTemp;

    #[test]
    fn get_iterations_cnst() {
        let cnst=CnstIterTemp {iterations:1000};
        assert_eq!(cnst.get_iterations(5),1000);
    }
}
