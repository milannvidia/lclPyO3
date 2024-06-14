pub mod always_true;
pub mod max_iterations;
pub mod max_sec;
pub mod min_temp;
pub mod multi_crit_and;
pub mod multi_crit_or;
pub mod must_improve;
pub mod no_improve;
pub mod terminationfunc;
pub use self::always_true::AlwaysTrue;
pub use self::max_iterations::MaxIterations;
pub use self::max_sec::MaxSec;
pub use self::min_temp::MinTemp;
pub use self::multi_crit_and::MultiCritAnd;
pub use self::multi_crit_or::MultiCritOr;
pub use self::must_improve::MustImprove;
pub use self::no_improve::NoImprove;
pub use self::terminationfunc::TerminationFunction;
