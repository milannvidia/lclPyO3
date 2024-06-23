pub trait IterationsTemperature: Send + Sync {
    fn get_iterations(&self, temp: usize) -> usize;
}
pub struct CnstIterTemp {
    iterations: usize,
}
impl CnstIterTemp {
    pub fn new(iterations: usize) -> Self {
        CnstIterTemp { iterations }
    }
}
impl IterationsTemperature for CnstIterTemp {
    fn get_iterations(&self, _temp: usize) -> usize {
        return self.iterations;
    }
}
