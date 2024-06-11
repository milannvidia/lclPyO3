pub trait IterationsTemperature: Send + Sync {
    fn get_iterations(&self, temp: usize) -> usize;
}
pub struct CnstIterTemp {
    pub iterations: usize,
}
impl IterationsTemperature for CnstIterTemp {
    fn get_iterations(&self, _temp: usize) -> usize {
        return self.iterations;
    }
}
