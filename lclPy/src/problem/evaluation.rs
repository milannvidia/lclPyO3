use crate::aidfunc::check_if_distance_matrix_symmetric;
use super::MoveType;
#[derive(Clone)]
pub enum Evaluation {
    Bins {
        weights: Vec<f64>,
        max_fill: f64,
    },
    EmptySpace {
        weights: Vec<f64>,
        max_fill: f64,
    },
    EmptySpaceExp {
        weights: Vec<f64>,
        max_fill: f64,
    },
    Tsp {
        distance_matrix: Vec<Vec<f64>>,
        symmetric: bool,
    },
    QAP {
        distance_matrix: Vec<Vec<f64>>,
        flow_matrix: Vec<Vec<f64>>,
    },
}
impl Evaluation {
    pub fn bins(weights: Vec<f64>, max_fill: f64) -> Evaluation {
        Evaluation::Bins { weights, max_fill }
    }
    pub fn empty_space(weights: Vec<f64>, max_fill: f64) -> Evaluation {
        Evaluation::EmptySpace { weights, max_fill }
    }
    pub fn empty_space_exp(weights: Vec<f64>, max_fill: f64) -> Evaluation {
        Evaluation::EmptySpaceExp { weights, max_fill }
    }
    pub fn tsp(distance_matrix: Vec<Vec<f64>>) -> Evaluation {
        let symmetric=check_if_distance_matrix_symmetric(&distance_matrix);
        Evaluation::Tsp {
            distance_matrix,
            symmetric,
        }
    }
    pub fn qap(distance_matrix: Vec<Vec<f64>>, flow_matrix: Vec<Vec<f64>>) -> Evaluation {
        Evaluation::QAP {
            distance_matrix,
            flow_matrix,
        }
    }

    pub(crate) fn delta_eval(
        &self,
        indices: (usize, usize),
        move_type: &MoveType,
        order: &mut Vec<usize>,
    ) -> f64 {
        match self {
            Evaluation::Bins { .. }
            | Evaluation::EmptySpace { .. }
            | Evaluation::EmptySpaceExp { .. } => {
                let first = self.eval(order);
                move_type.do_move(order, indices);
                let sec = self.eval(order);
                move_type.do_move(order, indices);
                sec - first
            }
            Evaluation::Tsp {
                distance_matrix,
                symmetric,
            } => {
                let mut init_score = 0.0;
                let mut next_score = 0.0;
                if matches!(move_type, MoveType::Swap { rng: _, size: _ })
                    || matches!(move_type, MoveType::Tsp { rng: _, size: _ })
                {
                    let from = indices.0;
                    let to = indices.1;
                    if from > 0 {
                        init_score += distance_matrix[order[from - 1]][order[from]];
                    } else {
                        init_score += distance_matrix[order[order.len() - 1]][order[from]];
                    }

                    init_score += distance_matrix[order[from]][order[from + 1]];
                    if from != to - 1 {
                        init_score += distance_matrix[order[to - 1]][order[to]];
                    }

                    init_score += distance_matrix[order[to]][order[(to + 1) % order.len()]];

                    move_type.do_move(order, indices);

                    if from > 0 {
                        next_score += distance_matrix[order[from - 1]][order[from]];
                    } else {
                        next_score += distance_matrix[order[order.len() - 1]][order[from]];
                    }

                    next_score += distance_matrix[order[from]][order[from + 1]];
                    if from != to - 1 {
                        next_score += distance_matrix[order[to - 1]][order[to]];
                    }
                    next_score += distance_matrix[order[to]][order[(to + 1) % order.len()]];

                    move_type.do_move(order, indices);
                } else {
                    if *symmetric {
                        if indices.0 > 0 {
                            init_score += distance_matrix[order[indices.0 - 1]][order[indices.0]];
                        } else {
                            init_score += distance_matrix[order[order.len() - 1]][order[indices.0]];
                        }
                        init_score +=
                            distance_matrix[order[indices.1]][order[(indices.1 + 1) % order.len()]];

                        move_type.do_move(order, indices);

                        if indices.0 > 0 {
                            next_score += distance_matrix[order[indices.0 - 1]][order[indices.0]];
                        } else {
                            next_score += distance_matrix[order[order.len() - 1]][order[indices.0]];
                        }
                        next_score +=
                            distance_matrix[order[indices.1]][order[(indices.1 + 1) % order.len()]];

                        move_type.do_move(order, indices);
                    } else {
                        for i in indices.0..indices.1 {
                            init_score += distance_matrix[order[i]][order[i + 1]];
                        }
                        if indices.0 > 0 {
                            init_score += distance_matrix[order[indices.0] - 1][order[indices.0]];
                        } else {
                            init_score += distance_matrix[order[order.len() - 1]][order[indices.0]];
                        }

                        init_score +=
                            distance_matrix[order[indices.1]][order[(indices.1 + 1) % order.len()]];
                        move_type.do_move(order, indices);
                        for i in indices.0..indices.1 {
                            next_score += distance_matrix[order[i]][order[i + 1]];
                        }
                        if indices.0 > 0 {
                            next_score += distance_matrix[order[indices.0] - 1][order[indices.0]];
                        } else {
                            next_score += distance_matrix[order[order.len() - 1]][order[indices.0]];
                        }
                        next_score +=
                            distance_matrix[order[indices.1]][order[(indices.1 + 1) % order.len()]];
                        move_type.do_move(order, indices);
                    }
                }
                next_score - init_score
            }
            Evaluation::QAP {
                distance_matrix,
                flow_matrix,
            } => {
                let d = distance_matrix;
                let f = flow_matrix;
                let p = order;
                let r = indices.0;
                let s = indices.1;
                let mut delta = 0.0;
                for i in 0..distance_matrix.len() {
                    if i == r || i == s {
                        continue;
                    }
                    delta += (d[s][i] - d[r][i]) * (f[p[r]][p[i]] - f[p[s]][p[i]]);
                }
                delta
            }
        }
    }

    pub(crate) fn eval(&self, order: &[usize]) -> f64 {
        match self {
            Evaluation::Bins { weights, max_fill } => {
                let mut score = 0.0;
                let mut fill_level = 0.0;
                for i in 0..order.len() {
                    if fill_level + weights[order[i]] > *max_fill {
                        score += 1.0;
                        fill_level = weights[order[i]];
                    } else {
                        fill_level += weights[order[i]];
                    }
                }
                score
            }
            Evaluation::EmptySpace { weights, max_fill } => {
                let mut score = 0.0;
                let mut fill_level = 0.0;
                for i in 0..order.len() {
                    if fill_level + weights[order[i]] > *max_fill {
                        score += max_fill - fill_level;
                        fill_level = weights[order[i]];
                    } else {
                        fill_level += weights[order[i]];
                    }
                }
                score += max_fill - fill_level;
                score
            }
            Evaluation::EmptySpaceExp { weights, max_fill } => {
                let mut score = 0.0;
                let mut fill_level = 0.0;
                for i in 0..order.len() {
                    if fill_level + weights[order[i]] > *max_fill {
                        score += (max_fill - fill_level).powf(2.0);
                        fill_level = weights[order[i]];
                    } else {
                        fill_level += weights[order[i]];
                    }
                }
                score += (max_fill - fill_level).powf(2.0);
                score
            }
            Evaluation::Tsp {
                distance_matrix,
                symmetric: _,
            } => {
                let mut score = 0.0;
                for i in 1..order.len() {
                    score += distance_matrix[order[i - 1]][order[i]];
                }
                score += distance_matrix[order[order.len() - 1]][order[0]];
                score
            }
            Evaluation::QAP {
                distance_matrix,
                flow_matrix,
            } => {
                let mut value = 0.0;
                for i in 0..distance_matrix.len() {
                    for j in (i + 1)..distance_matrix.len() {
                        value += distance_matrix[i][j] * flow_matrix[order[i]][order[j]];
                    }
                }
                value
            }
        }
    }
    pub(crate) fn length(&self) -> usize {
        match self {
            Evaluation::Bins {
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

#[cfg(test)]
mod tests {
    use std::vec;

    use rand::{rngs::SmallRng, SeedableRng};

    use crate::MoveType;

    use super::Evaluation;
    #[test]
    fn empty_space_test() {
        let eval = Evaluation::EmptySpace {
            weights: vec![2.0, 5.0, 4.0, 7.0, 1.0, 3.0, 8.0],
            max_fill: 10.0,
        };
        let swap_move = &MoveType::Swap {
            rng: Box::new(SmallRng::seed_from_u64(0)),
            size: 7,
        };
        let mut array: Vec<usize> = (0..7).collect();
        let score_0 = eval.eval(&array);
        let delta = eval.delta_eval((0, 3), swap_move, &mut array);
        swap_move.do_move(&mut array, (0, 3));
        let score_1 = eval.eval(&array);
        assert_eq!(score_0, 20.0);
        assert_eq!(delta, score_1 - score_0);
    }
    #[test]
    fn bins_test() {
        let eval = Evaluation::Bins {
            weights: vec![2.0, 5.0, 4.0, 7.0, 1.0, 3.0, 8.0],
            max_fill: 10.0,
        };
        let swap_move = &MoveType::Swap {
            rng: Box::new(SmallRng::seed_from_u64(0)),
            size: 7,
        };
        let mut array: Vec<usize> = (0..7).collect();
        let score_0 = eval.eval(&array);
        let delta = eval.delta_eval((0, 3), swap_move, &mut array);
        swap_move.do_move(&mut array, (0, 3));
        let score_1 = eval.eval(&array);
        assert_eq!(score_0, 4.0);
        assert_eq!(delta, score_1 - score_0);
    }
    #[test]
    fn empty_space_exp_test() {
        let eval = Evaluation::EmptySpaceExp {
            weights: vec![2.0, 5.0, 4.0, 7.0, 1.0, 3.0, 8.0],
            max_fill: 10.0,
        };
        let swap_move = &MoveType::Swap {
            rng: Box::new(SmallRng::seed_from_u64(0)),
            size: 7,
        };
        let mut array: Vec<usize> = (0..7).collect();
        let score_0 = eval.eval(&array);
        let delta = eval.delta_eval((0, 3), swap_move, &mut array);
        swap_move.do_move(&mut array, (0, 3));
        let score_1 = eval.eval(&array);
        assert_eq!(score_0, 102.0);
        assert_eq!(delta, score_1 - score_0);
    }
    #[test]
    fn tsp_test() {
        let distance_matrix: Vec<Vec<f64>> = vec![
            vec![0.0, 2.0, 5.0, 8.0],
            vec![2.0, 0.0, 4.0, 1.0],
            vec![5.0, 4.0, 0.0, 7.0],
            vec![8.0, 1.0, 7.0, 0.0],
        ];
        let eval = Evaluation::Tsp {
            distance_matrix,
            symmetric: true,
        };
        let swap_move = &MoveType::Swap {
            rng: Box::new(SmallRng::seed_from_u64(0)),
            size: 4,
        };
        let tests = vec![(1, 2), (0, 2), (0, 3)];
        let mut array: Vec<usize> = (0..4).collect();
        for test_move in tests {
            let score_0 = eval.eval(&array);
            let delta = eval.delta_eval(test_move, swap_move, &mut array);
            swap_move.do_move(&mut array, test_move);
            let score_1 = eval.eval(&array);
            swap_move.do_move(&mut array, test_move);
            assert_eq!(delta, score_1 - score_0);
        }
    }
    #[test]
    fn qap_test() {
        let distance_matrix: Vec<Vec<f64>> = vec![
            vec![0.0, 2.0, 9.0, 5.0],
            vec![2.0, 0.0, 4.0, 6.0],
            vec![9.0, 4.0, 0.0, 3.0],
            vec![5.0, 6.0, 3.0, 0.0],
        ];
        let flow_matrix = vec![
            vec![0.0, 2.0, 0.0, 0.0],
            vec![2.0, 0.0, 4.0, 0.0],
            vec![0.0, 4.0, 0.0, 8.0],
            vec![0.0, 0.0, 8.0, 0.0],
        ];
        let eval = Evaluation::QAP {
            distance_matrix,
            flow_matrix,
        };
        let swap_move = &MoveType::Swap {
            rng: Box::new(SmallRng::seed_from_u64(0)),
            size: 4,
        };
        let tests = vec![(1, 2), (0, 2), (0, 3)];
        let mut array: Vec<usize> = (0..4).collect();
        for test_move in tests {
            let score_0 = eval.eval(&array);
            let delta = eval.delta_eval(test_move, swap_move, &mut array);
            swap_move.do_move(&mut array, test_move);
            let score_1 = eval.eval(&array);
            swap_move.do_move(&mut array, test_move);
            assert_eq!(delta, score_1 - score_0);
        }
    }
}
