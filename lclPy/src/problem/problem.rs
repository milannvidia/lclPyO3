pub trait Problem: Send {
    fn get_mov(&mut self) -> (usize, usize);
    fn get_all_mov(&mut self) -> Vec<(usize, usize)>;
    fn do_mov(&mut self, indices: (usize, usize));
    fn delta_eval(&mut self, indices: (usize, usize)) -> isize;
    fn eval(&self) -> usize;
    fn reset(&mut self);
    fn set_best(&mut self);
    fn hash(&self) -> u64;
}
