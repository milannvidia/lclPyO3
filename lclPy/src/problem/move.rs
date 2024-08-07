use rand::{rngs::SmallRng, Rng, SeedableRng};
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

    pub fn do_move_multi(&self, array: &mut Vec<usize>, indices: (usize, (usize, usize))) {
        match self {
            MoveType::Reverse { rng: _, size: _ } => self.do_move(array, indices.1),
            MoveType::Swap { rng: _, size: _ } => self.do_move(array, indices.1),
            MoveType::Tsp { rng: _, size: _ } => self.do_move(array, indices.1),
            MoveType::MultiNeighbor {
                move_types,
                weights: _,
            } => move_types[0].do_move(array, indices.1),
        }
    }
    pub fn get_mov_multi(&mut self) -> (usize, (usize, usize)) {
        match self {
            MoveType::Reverse { rng: _, size: _ } => (0, self.get_mov()),
            MoveType::Swap { rng: _, size: _ } => (0, self.get_mov()),
            MoveType::Tsp { rng: _, size: _ } => (0, self.get_mov()),
            MoveType::MultiNeighbor {
                move_types,
                weights,
            } => {
                let random: f64 = SmallRng::from_entropy().r#gen();
                let mut sum: f64 = 0.0;
                for i in 0..weights.len() {
                    sum += weights[i];
                    if sum < random {
                        return (i, move_types[i].get_mov());
                    }
                }
                return (
                    move_types.len() - 1,
                    move_types[weights.len() - 1].get_mov(),
                );
            }
        }
    }

    pub fn get_all_mov_multi(&self) -> Vec<(usize, (usize, usize))> {
        match self {
            MoveType::Reverse { rng: _, size: _ } => self
                .get_all_mov()
                .into_iter()
                .map(|(a, b)| (0, (a, b)))
                .collect(),
            MoveType::Swap { rng: _, size: _ } => self
                .get_all_mov()
                .into_iter()
                .map(|(a, b)| (0, (a, b)))
                .collect(),
            MoveType::Tsp { rng: _, size: _ } => self
                .get_all_mov()
                .into_iter()
                .map(|(a, b)| (0, (a, b)))
                .collect(),
            MoveType::MultiNeighbor {
                move_types,
                weights: _,
            } => {
                let mut res: Vec<(usize, (usize, usize))> = vec![];
                for i in 0..move_types.len() {
                    res.extend(
                        move_types[i]
                            .get_all_mov()
                            .into_iter()
                            .map(|(a, b)| (i, (a, b))),
                    )
                }
                return res;
            }
        }
    }

    pub fn get_all_mov_select(&self, neighborhood_num: usize) -> Vec<(usize, (usize, usize))> {
        match self {
            MoveType::Reverse { rng: _, size: _ } => {
                if neighborhood_num > 0 {
                    panic!("neighborhoodNumber can't be higher then size-1")
                } else {
                    return self.get_all_mov_multi();
                }
            }
            MoveType::Swap { rng: _, size: _ } => {
                if neighborhood_num > 0 {
                    panic!("neighborhoodNumber can't be higher then size-1")
                } else {
                    return self.get_all_mov_multi();
                }
            }
            MoveType::Tsp { rng: _, size: _ } => {
                if neighborhood_num > 0 {
                    panic!("neighborhoodNumber can't be higher then size-1")
                } else {
                    return self.get_all_mov_multi();
                }
            }
            MoveType::MultiNeighbor {
                move_types,
                weights: _,
            } => {
                if neighborhood_num > move_types.len() - 1 {
                    panic!("neighborhoodNumber can't be higher then size-1")
                }
                return move_types[neighborhood_num].get_all_mov_multi();
            }
        }
    }
    pub fn get_mov_select(&mut self, neighborhood_num: usize) -> (usize, (usize, usize)) {
        match self {
            MoveType::Reverse { rng: _, size: _ } => {
                if neighborhood_num > 0 {
                    panic!("neighborhoodNumber can't be higher then size-1")
                } else {
                    return self.get_mov_multi();
                }
            }
            MoveType::Swap { rng: _, size: _ } => {
                if neighborhood_num > 0 {
                    panic!("neighborhoodNumber can't be higher then size-1")
                } else {
                    return self.get_mov_multi();
                }
            }
            MoveType::Tsp { rng: _, size: _ } => {
                if neighborhood_num > 0 {
                    panic!("neighborhoodNumber can't be higher then size-1")
                } else {
                    return self.get_mov_multi();
                }
            }
            MoveType::MultiNeighbor {
                move_types,
                weights: _,
            } => {
                if neighborhood_num > move_types.len() - 1 {
                    panic!("neighborhoodNumber can't be higher then size-1")
                }
                return move_types[neighborhood_num].get_mov_multi();
            }
        }
    }
}
