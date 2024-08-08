use super::MoveType;

pub trait Problem: Send {

    /// Get a random move
    ///
    /// # Examples
    ///
    /// ```
    ///# use rand::rngs::SmallRng;
    ///# use rand::SeedableRng;
    ///# use lclpy::problem::{ArrayProblem, Evaluation, Problem};
    ///# use lclpy::problem::MoveType::Tsp;
    ///# let distance_matrix=vec![
    ///     vec![0, 2, 5, 8],
    ///     vec![2, 0, 4, 1],
    ///     vec![5, 4, 0, 7],
    ///     vec![8, 1, 7, 0]];
    /// let mut problem = ArrayProblem::new(
    ///     &Tsp {rng:SmallRng::seed_from_u64(0),size:4},
    ///     &Evaluation::Tsp {distance_matrix,symmetric:true});
    ///
    /// assert_eq!((2,3), problem.get_mov())
    /// ```
    fn get_mov(&mut self) -> (usize, usize);

    /// Get all possible moves
    ///
    /// # Examples
    ///
    /// ```
    ///# use rand::rngs::SmallRng;
    ///# use rand::SeedableRng;
    ///# use lclpy::problem::{ArrayProblem, Evaluation, Problem};
    ///# use lclpy::problem::MoveType::Tsp;
    ///# let distance_matrix=vec![
    ///     vec![0, 2, 5, 8],
    ///     vec![2, 0, 4, 1],
    ///     vec![5, 4, 0, 7],
    ///     vec![8, 1, 7, 0]];
    /// let mut problem = ArrayProblem::new(
    ///     &Tsp {rng:SmallRng::seed_from_u64(0),size:4},
    ///     &Evaluation::Tsp {distance_matrix,symmetric:true});
    /// let solution:Vec<(usize,usize)>=vec![(1,2),(1,3),(2,3)];
    ///
    /// assert_eq!(solution, problem.get_all_mov())
    /// ```
    fn get_all_mov(&mut self) -> Vec<(usize, usize)>;

    /// Execute the given move
    ///
    /// # Examples
    ///
    /// ```
    ///# use rand::rngs::SmallRng;
    ///# use rand::SeedableRng;
    ///# use lclpy::problem::{ArrayProblem, Evaluation, Problem};
    ///# use lclpy::problem::MoveType::Tsp;
    ///# let distance_matrix=vec![
    ///     vec![0, 2, 5, 8],
    ///     vec![2, 0, 4, 1],
    ///     vec![5, 4, 0, 7],
    ///     vec![8, 1, 7, 0]];
    /// let mut problem = ArrayProblem::new(
    ///     &Tsp {rng:SmallRng::seed_from_u64(0),size:4},
    ///     &Evaluation::Tsp {distance_matrix,symmetric:true});
    /// problem.do_mov((1,2),None);
    ///
    /// assert_eq!(*problem.state(), [0,2,1,3])
    /// ```
    fn do_mov(&mut self, indices: (usize, usize), move_type: Option<&MoveType>);

    /// Gives the change in score if the given move would be performed.
    /// Also optimized. It only calculates what's necessary.
    ///
    /// # Examples
    ///
    /// ```
    ///# use rand::rngs::SmallRng;
    ///# use rand::SeedableRng;
    ///# use lclpy::problem::{ArrayProblem, Evaluation, Problem};
    ///# use lclpy::problem::MoveType::Tsp;
    ///# let distance_matrix=vec![
    ///     vec![0, 2, 5, 8],
    ///     vec![2, 0, 4, 1],
    ///     vec![5, 4, 0, 7],
    ///     vec![8, 1, 7, 0]];
    ///
    /// let mut problem = ArrayProblem::new(
    ///     &Tsp {rng:SmallRng::seed_from_u64(0),size:4},
    ///     &Evaluation::Tsp {distance_matrix,symmetric:true});
    /// let before=problem.eval();
    /// let res=problem.delta_eval((1,2),None);
    /// problem.do_mov((1,2),None);
    /// let after=problem.eval();
    ///
    /// assert_eq!(res, after as isize - before as isize)
    /// ```
    fn delta_eval(&mut self, indices: (usize, usize), move_type: Option<&MoveType>) -> isize;

    /// Calculates the current score.
    ///
    /// # Examples
    ///
    /// ```
    ///# use rand::rngs::SmallRng;
    ///# use rand::SeedableRng;
    ///# use lclpy::problem::{ArrayProblem, Evaluation, Problem};
    ///# use lclpy::problem::MoveType::Tsp;
    ///# let distance_matrix=vec![
    ///     vec![0, 2, 5, 8],
    ///     vec![2, 0, 4, 1],
    ///     vec![5, 4, 0, 7],
    ///     vec![8, 1, 7, 0]];
    /// let mut problem = ArrayProblem::new(
    ///     &Tsp {rng:SmallRng::seed_from_u64(0),size:4},
    ///     &Evaluation::Tsp {distance_matrix,symmetric:true});
    /// let before=problem.eval();
    /// let res=problem.delta_eval((1,2),None);
    /// problem.do_mov((1,2),None);
    /// let after=problem.eval();
    ///
    /// assert_eq!(res, after as isize - before as isize)
    /// ```
    fn eval(&self) -> usize;

    /// Resets the state to ascending indices.
    ///
    /// # Examples
    ///
    /// ```
    ///# use rand::rngs::SmallRng;
    ///# use rand::SeedableRng;
    ///# use lclpy::problem::{ArrayProblem, Evaluation, Problem};
    ///# use lclpy::problem::MoveType::Tsp;
    ///# let distance_matrix=vec![
    ///     vec![0, 2, 5, 8],
    ///     vec![2, 0, 4, 1],
    ///     vec![5, 4, 0, 7],
    ///     vec![8, 1, 7, 0]];
    /// let mut problem = ArrayProblem::new(
    ///     &Tsp {rng:SmallRng::seed_from_u64(0),size:4},
    ///     &Evaluation::Tsp {distance_matrix,symmetric:true});
    /// problem.do_mov((1,2),None);
    /// problem.reset();
    ///
    /// assert_eq!(*problem.state(), [0,1,2,3])
    /// ```
    fn reset(&mut self);

    /// Sets the current state as best order.
    ///
    /// # Examples
    ///
    /// ```
    ///# use rand::rngs::SmallRng;
    ///# use rand::SeedableRng;
    ///# use lclpy::problem::{ArrayProblem, Evaluation, Problem};
    ///# use lclpy::problem::MoveType::Tsp;
    ///# let distance_matrix=vec![
    ///     vec![0, 2, 5, 8],
    ///     vec![2, 0, 4, 1],
    ///     vec![5, 4, 0, 7],
    ///     vec![8, 1, 7, 0]];
    /// let mut problem = ArrayProblem::new(
    ///     &Tsp {rng:SmallRng::seed_from_u64(0),size:4},
    ///     &Evaluation::Tsp {distance_matrix,symmetric:true});
    /// problem.do_mov((1,2),None);
    /// problem.set_best();
    ///
    /// assert_eq!(*problem.best_solution(), [0,2,1,3])
    /// ```
    fn set_best(&mut self);

    /// Gives a hash of the current state. Used in tabu search.
    ///
    /// # Examples
    ///
    /// ```
    ///# use rand::rngs::SmallRng;
    ///# use rand::SeedableRng;
    ///# use lclpy::problem::{ArrayProblem, Evaluation, Problem};
    ///# use lclpy::problem::MoveType::Tsp;
    ///# let distance_matrix=vec![
    ///     vec![0, 2, 5, 8],
    ///     vec![2, 0, 4, 1],
    ///     vec![5, 4, 0, 7],
    ///     vec![8, 1, 7, 0]];
    /// let mut problem = ArrayProblem::new(
    ///     &Tsp {rng:SmallRng::seed_from_u64(0),size:4},
    ///     &Evaluation::Tsp {distance_matrix,symmetric:true});
    ///
    /// assert_eq!(problem.hash(), 9144871353323486087)
    /// ```
    fn hash(&self) -> u64;

    /// Gives the move-type the current problem uses. Used for variable neighborhood search.
    fn get_move_type(&self) -> &MoveType;
}
