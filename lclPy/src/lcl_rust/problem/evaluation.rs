use super::MoveType;

pub enum Evaluation {
    EmptyBins {
        weights: Vec<usize>,
        max_fill: usize,
    },
    EmptySpace {
        weights: Vec<usize>,
        max_fill: usize,
    },
    EmptySpaceExp {
        weights: Vec<usize>,
        max_fill: usize,
    },
    Tsp {
        weights: Vec<Vec<usize>>,
        symmetric: bool,
    },
}
impl Evaluation {
    pub(crate) fn delta_eval(
        &self,
        indices: (usize, usize),
        move_type: &MoveType,
        solution: &mut Vec<usize>,
    ) -> isize {
        match self {
            Evaluation::EmptyBins {
                weights: _,
                max_fill: _,
            }
            | Evaluation::EmptySpace {
                weights: _,
                max_fill: _,
            }
            | Evaluation::EmptySpaceExp {
                weights: _,
                max_fill: _,
            } => {
                let first = self.eval(solution);
                move_type.do_move(solution, indices);
                let sec = self.eval(solution);
                move_type.do_move(solution, indices);
                return (sec - first) as isize;
            }
            Evaluation::Tsp { weights, symmetric } => {
                let mut init_score = 0;
                let mut next_score = 0;
                if matches!(move_type, MoveType::Swap { rng: _, size: _ })
                    || matches!(move_type, MoveType::Tsp { rng: _, size: _ })
                {
                    if indices.0 > 0 {
                        init_score += weights[solution[indices.0 - 1]][solution[indices.0]];
                    }
                    init_score += weights[solution[indices.0]][solution[indices.0 + 1]];
                    init_score += weights[solution[indices.1 - 1]][solution[indices.1]];
                    init_score +=
                        weights[solution[indices.1]][solution[(indices.1 + 1) % solution.len()]];

                    move_type.do_move(solution, indices);

                    if indices.0 > 0 {
                        next_score += weights[solution[indices.0 - 1]][solution[indices.0]];
                    }
                    next_score += weights[solution[indices.0]][solution[indices.0 + 1]];
                    next_score += weights[solution[indices.1 - 1]][solution[indices.1]];
                    next_score +=
                        weights[solution[indices.1]][solution[(indices.1 + 1) % solution.len()]];

                    move_type.do_move(solution, indices);
                } else {
                    if *symmetric {
                        if indices.0 > 0 {
                            init_score += weights[solution[indices.0 - 1]][solution[indices.0]];
                        }
                        if indices.1 < solution.len() {
                            init_score += weights[solution[indices.1]]
                                [solution[(indices.1 + 1) % solution.len()]];
                        }
                        move_type.do_move(solution, indices);
                        if indices.0 > 0 {
                            next_score += weights[solution[indices.0 - 1]][solution[indices.0]];
                        }
                        if indices.1 < solution.len() {
                            next_score += weights[solution[indices.1]]
                                [solution[(indices.1 + 1) % solution.len()]];
                        }
                        move_type.do_move(solution, indices);
                    } else {
                        for i in indices.0..indices.1 {
                            init_score += weights[solution[i]][solution[i + 1]];
                        }
                        init_score += weights[solution[indices.1]]
                            [solution[(indices.1 + 1) % solution.len()]];
                        move_type.do_move(solution, indices);
                        for i in indices.0..indices.1 {
                            next_score += weights[solution[i]][solution[i + 1]];
                        }
                        next_score += weights[solution[indices.1]]
                            [solution[(indices.1 + 1) % solution.len()]];
                        move_type.do_move(solution, indices);
                    }
                }
                next_score as isize - init_score as isize
            }
        }
    }

    pub(crate) fn eval(&self, solution: &[usize]) -> usize {
        match self {
            Evaluation::EmptyBins { weights, max_fill } => {
                let mut score = 0usize;
                let mut filllevel = 0usize;
                for i in 0..solution.len() {
                    if filllevel + weights[solution[i]] > *max_fill {
                        score += 1;
                        filllevel = 0;
                    } else {
                        filllevel += weights[solution[i]];
                    }
                }
                return score;
            }
            Evaluation::EmptySpace { weights, max_fill } => {
                let mut score = 0usize;
                let mut filllevel = 0usize;
                for i in 0..solution.len() {
                    if filllevel + weights[solution[i]] > *max_fill {
                        score += max_fill - filllevel;
                        filllevel = 0;
                    } else {
                        filllevel += weights[solution[i]];
                    }
                }
                return score;
            }
            Evaluation::EmptySpaceExp { weights, max_fill } => {
                let mut score = 0usize;
                let mut filllevel = 0usize;
                for i in 0..solution.len() {
                    if filllevel + weights[solution[i]] > *max_fill {
                        score += (max_fill - filllevel).pow(2);
                        filllevel = 0;
                    } else {
                        filllevel += weights[solution[i]];
                    }
                }
                return score;
            }
            Evaluation::Tsp {
                weights,
                symmetric: _,
            } => {
                let mut score: usize = 0;
                for i in 1..solution.len() {
                    score += weights[solution[i - 1]][solution[i]];
                }
                score += weights[solution[solution.len() - 1]][solution[0]];
                return score;
            }
        }
    }
}
