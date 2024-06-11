pub trait Problem: Send {
    fn mov(&mut self) -> (usize, usize);
    fn all_mov(&mut self) -> Vec<(usize, usize)>;
    fn domov(&mut self, indices: (usize, usize));
    fn delta(&mut self, indices: (usize, usize)) -> isize;
    fn eval(&self) -> usize;
    fn reset(&mut self);
    fn set_best(&mut self);
}
