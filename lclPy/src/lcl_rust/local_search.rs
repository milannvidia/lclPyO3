pub trait LocalSearch: Send {
    fn reset(&mut self);
    fn run(&mut self, log: bool) -> Vec<(u128, isize, isize, usize)>;
}
