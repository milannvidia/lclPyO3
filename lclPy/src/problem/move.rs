use rand::{rngs::SmallRng, Rng, SeedableRng};
#[derive(Clone)]
pub enum MoveType {
    Reverse {
        rng: SmallRng,
        size: usize,
    },
    Swap {
        rng: SmallRng,
        size: usize,
    },
    Tsp {
        rng: SmallRng,
        size: usize,
    },
    MultiNeighbor {
        move_types: Vec<MoveType>,
        weights: Vec<f64>,
    },
}
impl MoveType {
    pub fn do_move(&self, array: &mut Vec<usize>, indices: (usize, usize)) {
        match self {
            MoveType::Reverse { rng: _, size: _ } => {
                for i in 0..(indices.1 - indices.0 + 1) / 2 {
                    array.swap(indices.0 + i, indices.1 - i);
                }
            }
            MoveType::Swap { rng: _, size: _ } | MoveType::Tsp { rng: _, size: _ } => {
                array.swap(indices.0, indices.1);
            }
            MoveType::MultiNeighbor {
                move_types: _,
                weights: _,
            } => {
                panic!("MultiNeighbor doesn't support do_move")
            }
        }
    }
    pub fn get_mov(&mut self) -> (usize, usize) {
        match self {
            MoveType::Reverse { rng, size } | MoveType::Swap { rng, size } => {
                let i = rng.gen_range(0..*size);
                let mut j = rng.gen_range(1..*size);
                while i == j {
                    j = rng.gen_range(1..*size);
                }
                if j < i {
                    return (j, i);
                }
                (i, j)
            }
            MoveType::Tsp { rng, size } => {
                let i = rng.gen_range(1..*size);
                let mut j = rng.gen_range(2..*size);
                while i == j {
                    j = rng.gen_range(1..*size);
                }
                if j < i {
                    return (j, i);
                }
                (i, j)
            }
            MoveType::MultiNeighbor {
                move_types: _,
                weights: _,
            } => {
                panic!("MultiNeighbor doesn't support get_move");
            }
        }
    }
    pub fn get_all_mov(&self) -> Vec<(usize, usize)> {
        match self {
            MoveType::Reverse { rng: _, size } | MoveType::Swap { rng: _, size } => {
                let mut moves: Vec<(usize, usize)> = vec![];
                for i in 0..(*size - 1) {
                    for j in (i + 1)..*size {
                        moves.push((i, j))
                    }
                }
                moves
            }
            MoveType::Tsp { rng: _, size } => {
                let mut moves: Vec<(usize, usize)> = vec![];
                for i in 1..(*size - 1) {
                    for j in (i + 1)..*size {
                        moves.push((i, j))
                    }
                }
                moves
            }
            MoveType::MultiNeighbor {
                move_types: _,
                weights: _,
            } => {
                panic!("MultiNeighbor doesn't support get_all_mov")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::panic;

    use rand::{rngs::SmallRng, SeedableRng};

    use crate::MoveType;
    #[test]
    fn reverse_move_type_test() {
        let mut reverse = MoveType::Reverse {
            rng: SmallRng::seed_from_u64(0),
            size: 4,
        };
        assert_eq!(reverse.get_mov(), (2, 3));
        assert_eq!(
            reverse.get_all_mov(),
            [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]
        );

        let mut array: Vec<usize> = vec![0, 1, 2, 3];
        reverse.do_move(&mut array, (0, 3));
        assert_eq!(array, [3, 2, 1, 0])
    }
    #[test]
    fn tsp_move_type_test() {
        let mut tsp = MoveType::Tsp {
            rng: SmallRng::seed_from_u64(0),
            size: 4,
        };
        assert_eq!(tsp.get_mov(), (2, 3));
        assert_eq!(tsp.get_all_mov(), [(1, 2), (1, 3), (2, 3)]);

        let mut array: Vec<usize> = vec![0, 1, 2, 3];
        tsp.do_move(&mut array, (0, 3));
        assert_eq!(array, [3, 1, 2, 0])
    }
    #[test]
    fn swap_move_type_test() {
        let mut swap = MoveType::Swap {
            rng: SmallRng::seed_from_u64(0),
            size: 4,
        };
        assert_eq!(swap.get_mov(), (2, 3));
        assert_eq!(
            swap.get_all_mov(),
            [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]
        );

        let mut array: Vec<usize> = vec![0, 1, 2, 3];
        swap.do_move(&mut array, (0, 3));
        assert_eq!(array, [3, 1, 2, 0])
    }
    #[test]
    fn multi_move_type_test() {
        let mut multi = MoveType::MultiNeighbor {
            move_types: vec![],
            weights: vec![],
        };
        let get_all_mov = panic::catch_unwind(|| multi.get_all_mov());
        let get_mov = panic::catch_unwind(|| multi.clone().get_mov());
        let get_do_mov = panic::catch_unwind(|| {
            let mut array: Vec<usize> = vec![0, 1, 2, 3];
            multi.clone().do_move(&mut array, (0, 1))
        });
        assert!(get_all_mov.is_err());
        assert!(get_mov.is_err());
        assert!(get_do_mov.is_err());
    }
}
