#[derive(Clone)]
pub enum IterationsTemperature {
    ConstIterTemp { iterations: usize },
}
impl IterationsTemperature {
    /// How many iterations for a given temperature
    ///
    /// # Arguments
    ///
    /// * `temp`: current temp
    ///
    /// returns: runs in this temperature range
    ///
    /// # Examples
    ///
    /// ```
    /// use lclpy::local_search::simulated_annealing::IterationsTemperature::ConstIterTemp;
    /// let constant = ConstIterTemp {iterations:1000};
    /// assert_eq!(constant.get_iterations(5), 1000);
    /// ```
    pub fn get_iterations(&self, _temp: usize) -> usize {
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
