use lclpy::termination::{MaxSec, MinTemp, TerminationFunction};

pub trait TerminationFunction: Send {

    /// Checks whether the termination criterion has been reached.
    ///
    /// # Examples
    ///
    /// ```
    ///# use lclpy::termination::{MaxIterations, TerminationFunction};
    /// let mut termination_crit = MaxIterations::new(1);
    /// assert!(termination_crit.keep_running());
    /// termination_crit.iteration_done();
    /// assert!(!termination_crit.keep_running())
    /// ```
    fn keep_running(&self) -> bool;

    /// Initializes the Termination function. Can also be seen as a reset.
    ///
    /// # Examples
    ///
    /// ```
    ///# use lclpy::termination::{MaxIterations, TerminationFunction};
    /// let mut termination_crit = MaxIterations::new(1);
    /// assert!(termination_crit.keep_running());
    /// termination_crit.iteration_done();
    /// termination_crit.init();
    /// assert!(termination_crit.keep_running())
    /// ```
    fn init(&mut self);

    /// Checks a variable, only used for simulated annealing.
    ///
    /// # Examples
    ///
    /// ```
    ///# use lclpy::termination::{MinTemp, TerminationFunction};
    /// let mut termination_crit = MinTemp::new(50);
    /// assert!(termination_crit.check_variable(60));
    /// ```
    fn check_variable(&mut self, var: isize) -> bool;

    /// Checks a new-found score, used for vns.
    ///
    /// # Examples
    ///
    /// ```
    ///# use lclpy::termination::{MustImprove, TerminationFunction};
    /// let mut termination_crit = MustImprove::new(true);
    /// assert!(termination_crit.check_new_variable(60));
    /// assert!(!termination_crit.check_new_variable(60));
    /// ```
    fn check_new_variable(&mut self, var: isize);

    /// Updates the termination function of an iteration. Used for MaxIterations.
    ///
    /// # Examples
    ///
    /// ```
    ///# use lclpy::termination::{MustImprove, TerminationFunction};
    /// let mut termination_crit = MustImprove::new(true);
    /// assert!(termination_crit.check_new_variable(60));
    /// assert!(!termination_crit.check_new_variable(60));
    /// ```
    fn iteration_done(&mut self);
}
