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
        order: &mut Vec<usize>,
    ) -> isize {
        match self {
            Evaluation::EmptyBins { .. }
            | Evaluation::EmptySpace { .. }
            | Evaluation::EmptySpaceExp { .. } => {
                let first = self.eval(order);
                move_type.do_move(order, indices);
                let sec = self.eval(order);
                move_type.do_move(order, indices);
                sec as isize - first as isize
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
                next_score as isize - init_score as isize
            }
            Evaluation::QAP {
                distance_matrix,
                flow_matrix,
            } => {
                // let before = self.eval(&order);
                // move_type.do_move(order, indices);
                // let after = self.eval(order);
                // move_type.do_move(order, indices);
                // after as isize - before as isize
                //TODO: debug
                let d = distance_matrix;
                let f = flow_matrix;
                let p = order;
                let r = indices.0;
                let s = indices.1;
                let mut delta = 0;
                for i in 0..distance_matrix.len() {
                    if i == r || i == s {
                        continue;
                    }
                    delta += (d[s][i] as isize - d[r][i] as isize)
                        * (f[p[r]][p[i]] as isize - f[p[s]][p[i]] as isize);
                }

                delta
            }
        }
    }

    pub(crate) fn eval(&self, order: &[usize]) -> usize {
        match self {
            Evaluation::EmptyBins { weights, max_fill } => {
                let mut score = 0usize;
                let mut fill_level = 0usize;
                for i in 0..order.len() {
                    if fill_level + weights[order[i]] > *max_fill {
                        score += 1;
                        fill_level = weights[order[i]];
                    } else {
                        fill_level += weights[order[i]];
                    }
                }
                score
            }
            Evaluation::EmptySpace { weights, max_fill } => {
                let mut score = 0usize;
                let mut fill_level = 0usize;
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
                let mut score = 0usize;
                let mut fill_level = 0usize;
                for i in 0..order.len() {
                    if fill_level + weights[order[i]] > *max_fill {
                        score += (max_fill - fill_level).pow(2);
                        fill_level = weights[order[i]];
                    } else {
                        fill_level += weights[order[i]];
                    }
                }
                score += (max_fill - fill_level).pow(2);
                score
            }
            Evaluation::Tsp {
                distance_matrix,
                symmetric: _,
            } => {
                let mut score: usize = 0;
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
                let mut value = 0;
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

#[cfg(test)]
mod tests {
    use std::vec;

    use rand::{rngs::SmallRng, SeedableRng};

    use crate::MoveType;

    use super::Evaluation;
    #[test]
    fn empty_space_test() {
        let eval = Evaluation::EmptySpace {
            weights: vec![2, 5, 4, 7, 1, 3, 8],
            max_fill: 10,
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
        assert_eq!(score_0, 20);
        assert_eq!(delta, score_1 as isize - score_0 as isize);
    }
    #[test]
    fn bins_test() {
        let eval = Evaluation::EmptyBins {
            weights: vec![2, 5, 4, 7, 1, 3, 8],
            max_fill: 10,
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
        assert_eq!(score_0, 4);
        assert_eq!(delta, score_1 as isize - score_0 as isize);
    }
    #[test]
    fn empty_space_exp_test() {
        let eval = Evaluation::EmptySpaceExp {
            weights: vec![2, 5, 4, 7, 1, 3, 8],
            max_fill: 10,
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
        assert_eq!(score_0, 102);
        assert_eq!(delta, score_1 as isize - score_0 as isize);
    }
    #[test]
    fn tsp_test() {
        let distance_matrix: Vec<Vec<usize>> = vec![
            vec![0, 2, 5, 8],
            vec![2, 0, 4, 1],
            vec![5, 4, 0, 7],
            vec![8, 1, 7, 0],
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
            assert_eq!(delta, score_1 as isize - score_0 as isize);
        }
    }
    #[test]
    fn qap_test() {
        let distance_matrix: Vec<Vec<usize>> = vec![
            vec![0, 2, 9, 5],
            vec![2, 0, 4, 6],
            vec![9, 4, 0, 3],
            vec![5, 6, 3, 0],
        ];
        let flow_matrix = vec![
            vec![0, 2, 0, 0],
            vec![2, 0, 4, 0],
            vec![0, 4, 0, 8],
            vec![0, 0, 8, 0],
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
            assert_eq!(delta, score_1 as isize - score_0 as isize);
        }
    }
}
