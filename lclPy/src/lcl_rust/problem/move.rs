use rand::Rng;
#[derive(Clone)]
pub enum MoveType {
    Reverse {
        rng: rand::rngs::SmallRng,
        size: usize,
    },
    Swap {
        rng: rand::rngs::SmallRng,
        size: usize,
    },
    Tsp {
        rng: rand::rngs::SmallRng,
        size: usize,
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
                return (i, j);
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
                return (i, j);
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
        }
    }
}
