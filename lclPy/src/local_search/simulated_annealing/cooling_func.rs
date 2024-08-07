// pub trait CoolingFunction: Send + Sync {
//     fn get_next_temp(&self, temp: usize) -> usize;
// }
// pub struct GeometricCooling {
//     pub alpha: f64,
// }
// impl CoolingFunction for GeometricCooling {
//     fn get_next_temp(&self, temp: usize) -> usize {
//         let result = self.alpha * temp as f64;
//         return result.round() as usize;
//     }
// }
#[derive(Clone)]
pub enum CoolingFunction{
    GeometricCooling{
        alpha:f64
    }
}
impl CoolingFunction{
    pub(crate) fn get_next_temp(&self,temp:usize)-> usize{
        match &self{
            CoolingFunction::GeometricCooling { alpha } => {
                let result = alpha * temp as f64;
                 return result.round() as usize;
            },
        }
    }
}
