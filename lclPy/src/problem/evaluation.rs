use super::MoveType;
#[derive(Clone)]
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
        distance_matrix: Vec<Vec<usize>>,
        symmetric: bool,
    },
    QAP {
        distance_matrix: Vec<Vec<usize>>,
        flow_matrix: Vec<Vec<usize>>,
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
            Evaluation::EmptyBins { .. }
            | Evaluation::EmptySpace { .. }
            | Evaluation::EmptySpaceExp { .. } => {
                let first = self.eval(solution);
                move_type.do_move(solution, indices);
                let sec = self.eval(solution);
                move_type.do_move(solution, indices);
                return sec as isize - first as isize;
            }
            Evaluation::Tsp {
                distance_matrix,
                symmetric,
            } => {
                let mut init_score = 0;
                let mut next_score = 0;
                if matches!(move_type, MoveType::Swap { rng: _, size: _ })
                    || matches!(move_type, MoveType::Tsp { rng: _, size: _ })
                {
                    let from = indices.0;
                    let to = indices.1;
                    if from > 0 {
                        init_score += distance_matrix[solution[from - 1]][solution[from]];
                    } else {
                        init_score += distance_matrix[solution[solution.len() - 1]][solution[from]];
                    }

                    init_score += distance_matrix[solution[from]][solution[from + 1]];
                    if from != to - 1 {
                        init_score += distance_matrix[solution[to - 1]][solution[to]];
                    }

                    init_score +=
                        distance_matrix[solution[to]][solution[(to + 1) % solution.len()]];

                    move_type.do_move(solution, indices);

                    if from > 0 {
                        next_score += distance_matrix[solution[from - 1]][solution[from]];
                    } else {
                        next_score += distance_matrix[solution[solution.len() - 1]][solution[from]];
                    }

                    next_score += distance_matrix[solution[from]][solution[from + 1]];
                    if from != to - 1 {
                        next_score += distance_matrix[solution[to - 1]][solution[to]];
                    }
                    next_score +=
                        distance_matrix[solution[to]][solution[(to + 1) % solution.len()]];

                    move_type.do_move(solution, indices);
                } else {
                    if *symmetric {
                        if indices.0 > 0 {
                            init_score +=
                                distance_matrix[solution[indices.0 - 1]][solution[indices.0]];
                        } else {
                            init_score +=
                                distance_matrix[solution[solution.len() - 1]][solution[indices.0]];
                        }
                        init_score += distance_matrix[solution[indices.1]]
                            [solution[(indices.1 + 1) % solution.len()]];
                        move_type.do_move(solution, indices);
                        if indices.0 > 0 {
                            next_score +=
                                distance_matrix[solution[indices.0 - 1]][solution[indices.0]];
                        } else {
                            next_score +=
                                distance_matrix[solution[solution.len() - 1]][solution[indices.0]];
                        }

                        next_score += distance_matrix[solution[indices.1]]
                            [solution[(indices.1 + 1) % solution.len()]];

                        move_type.do_move(solution, indices);
                    } else {
                        for i in indices.0..indices.1 {
                            init_score += distance_matrix[solution[i]][solution[i + 1]];
                        }
                        if indices.0 > 0 {
                            init_score +=
                                distance_matrix[solution[indices.0] - 1][solution[indices.0]];
                        } else {
                            init_score +=
                                distance_matrix[solution[solution.len() - 1]][solution[indices.0]];
                        }

                        init_score += distance_matrix[solution[indices.1]]
                            [solution[(indices.1 + 1) % solution.len()]];
                        move_type.do_move(solution, indices);
                        for i in indices.0..indices.1 {
                            next_score += distance_matrix[solution[i]][solution[i + 1]];
                        }
                        if indices.0 > 0 {
                            next_score +=
                                distance_matrix[solution[indices.0] - 1][solution[indices.0]];
                        } else {
                            next_score +=
                                distance_matrix[solution[solution.len() - 1]][solution[indices.0]];
                        }
                        next_score += distance_matrix[solution[indices.1]]
                            [solution[(indices.1 + 1) % solution.len()]];
                        move_type.do_move(solution, indices);
                    }
                }
                next_score as isize - init_score as isize
            }
            Evaluation::QAP {
                distance_matrix,
                flow_matrix,
            } => {
                //TODO: check of het werkt
                let mut delta_f: isize = 0;
                let A = distance_matrix;
                let B = flow_matrix;
                let pi = solution;
                let k = indices.0;
                let l = indices.1;

                for i in 0..distance_matrix.len() {
                    if i != indices.0 && i != indices.1 {
                        delta_f += A[k][i] as isize
                            * (B[pi[l]][pi[i]] as isize - B[pi[k]][pi[i]] as isize);
                        delta_f += A[i][k] as isize
                            * (B[pi[i]][pi[l]] as isize - B[pi[i]][pi[k]] as isize);
                        delta_f += A[l][i] as isize
                            * (B[pi[k]][pi[i]] as isize - B[pi[l]][pi[i]] as isize);
                        delta_f += A[i][l] as isize
                            * (B[pi[i]][pi[k]] as isize - B[pi[i]][pi[l]] as isize);
                    }
                }
                delta_f += A[k][l] as isize * (B[pi[l]][pi[k]] as isize - B[pi[k]][pi[l]] as isize);
                delta_f += A[l][k] as isize * (B[pi[k]][pi[l]] as isize - B[pi[l]][pi[k]] as isize);

                return 2 * delta_f;
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
                distance_matrix,
                symmetric: _,
            } => {
                let mut score: usize = 0;
                for i in 1..solution.len() {
                    score += distance_matrix[solution[i - 1]][solution[i]];
                }
                score += distance_matrix[solution[solution.len() - 1]][solution[0]];
                return score;
            }
            Evaluation::QAP {
                distance_matrix,
                flow_matrix,
            } => {
                let mut value = 0;
                for i in 0..distance_matrix.len() {
                    for j in (i + 1)..distance_matrix.len() {
                        value += distance_matrix[i][j] * flow_matrix[solution[i]][solution[j]];
                    }
                }
                value
            }
        }
    }
    pub(crate) fn length(&self) -> usize {
        match self {
            Evaluation::EmptyBins {
                weights,
                max_fill: _,
            } => weights.len(),
            Evaluation::EmptySpace {
                weights,
                max_fill: _,
            } => weights.len(),
            Evaluation::EmptySpaceExp {
                weights,
                max_fill: _,
            } => weights.len(),
            Evaluation::Tsp {
                distance_matrix,
                symmetric: _,
            } => distance_matrix.len(),
            Evaluation::QAP {
                distance_matrix,
                flow_matrix: _,
            } => distance_matrix.len(),
        }
    }
}
