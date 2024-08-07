// pub trait IterationsTemperature: Send + Sync {
//     fn get_iterations(&self, temp: usize) -> usize;
// }
// pub struct CnstIterTemp {
//     iterations: usize,
// }
// impl CnstIterTemp {
//     pub fn new(iterations: usize) -> Self {
//         CnstIterTemp { iterations }
//     }
// }
// impl IterationsTemperature for CnstIterTemp {
//     fn get_iterations(&self, _temp: usize) -> usize {
//         return self.iterations;
//     }
// }

#[derive(Clone)]
pub enum IterationsTemperature {
    CnstIterTemp { iterations: usize },
}
impl IterationsTemperature {
    pub(crate) fn get_iterations(&self, _temp: usize) -> usize {
        match &self {
            IterationsTemperature::CnstIterTemp { iterations } => {
                return *iterations;
            }
        }
    }
}
