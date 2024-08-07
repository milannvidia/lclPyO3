use super::MoveType;

pub trait Problem: Send {
    fn get_mov(&mut self) -> (usize, usize);
    fn get_all_mov(&mut self) -> Vec<(usize, usize)>;
    fn do_mov(&mut self, indices: (usize, usize), move_type: Option<&MoveType>);
    fn delta_eval(&mut self, indices: (usize, usize), move_type: Option<&MoveType>) -> isize;
    fn eval(&self) -> usize;
    fn reset(&mut self);
    fn set_best(&mut self);
    fn hash(&self) -> u64;
    fn get_move_type(&self) -> &MoveType;
}
