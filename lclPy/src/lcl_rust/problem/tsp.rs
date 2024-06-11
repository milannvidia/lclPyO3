use super::Problem;
use rand::{rngs::SmallRng, Rng, SeedableRng};

pub struct TSP {
    //otherwise reverse array
    pub(crate) swap: bool,
    pub(crate) distance_matrix: Vec<Vec<usize>>,
    pub(crate) solution: Vec<usize>,
    pub(crate) size: usize,
    pub(crate) rng: rand::rngs::SmallRng,
    pub(crate) best_solution: Vec<usize>,
}
impl TSP {
    pub fn new(swap: bool, distance_matrix: Vec<Vec<usize>>, seed: Option<u64>) -> Self {
        let x = distance_matrix.len();
        let rng: SmallRng;
        if seed.is_none() {
            rng = rand::rngs::SmallRng::from_entropy();
        } else {
            rng = rand::rngs::SmallRng::seed_from_u64(seed.unwrap());
        }
        TSP {
            swap,
            distance_matrix,
            solution: (0..x).collect(),
            size: x,
            rng,
            best_solution: (0..x).collect(),
        }
    }
}
impl Problem for TSP {
    fn mov(&mut self) -> (usize, usize) {
        let i = self.rng.gen_range(1..self.size);
        let mut j = self.rng.gen_range(1..self.size);
        while i == j {
            j = self.rng.gen_range(1..self.size);
        }
        if j < i {
            return (j, i);
        }
        return (i, j);
    }

    fn all_mov(&mut self) -> Vec<(usize, usize)> {
        let mut moves = vec![];
        for i in 1..self.size - 1 {
            for j in i + 1..self.size {
                moves.append(&mut vec![(i, j)])
            }
        }
        return moves;
    }

    fn domov(&mut self, indices: (usize, usize)) {
        if self.swap {
            self.solution.swap(indices.0, indices.1);
        } else {
            for i in 0..(indices.1 - indices.0 + 1) / 2 {
                self.solution.swap(indices.0 + i, indices.1 - i);
            }
        }
    }

    fn delta(&mut self, indices: (usize, usize)) -> isize {
        let mut initialscore = 0;
        let mut nextscore = 0;
        return if self.swap {
            let indexsafe = (indices.1 + 1) % self.size;
            initialscore +=
                self.distance_matrix[self.solution[indices.0 - 1]][self.solution[indices.0]];
            initialscore +=
                self.distance_matrix[self.solution[indices.0]][self.solution[indices.0 + 1]];
            initialscore +=
                self.distance_matrix[self.solution[indices.1 - 1]][self.solution[indices.1]];
            initialscore +=
                self.distance_matrix[self.solution[indices.1]][self.solution[indexsafe]];

            self.solution.swap(indices.0, indices.1);

            nextscore +=
                self.distance_matrix[self.solution[indices.0 - 1]][self.solution[indices.0]];
            nextscore +=
                self.distance_matrix[self.solution[indices.0]][self.solution[indices.0 + 1]];
            nextscore +=
                self.distance_matrix[self.solution[indices.1 - 1]][self.solution[indices.1]];
            nextscore += self.distance_matrix[self.solution[indices.1]][self.solution[indexsafe]];
            self.solution.swap(indices.0, indices.1);

            nextscore as isize - initialscore as isize
        } else {
            //FIXME: cuurently uses idea of undirected graphs
            let indexsafe = (indices.1 + 1) % self.size;

            initialscore +=
                self.distance_matrix[self.solution[indices.0 - 1]][self.solution[indices.0]];
            initialscore +=
                self.distance_matrix[self.solution[indices.1]][self.solution[indexsafe]];

            self.solution.swap(indices.0, indices.1);

            nextscore +=
                self.distance_matrix[self.solution[indices.0 - 1]][self.solution[indices.0]];
            nextscore += self.distance_matrix[self.solution[indices.1]][self.solution[indexsafe]];

            self.solution.swap(indices.0, indices.1);
            nextscore as isize - initialscore as isize
        };
    }

    fn eval(&self) -> usize {
        let mut score = 0;
        for i in 1..self.size {
            score += self.distance_matrix[self.solution[i - 1]][self.solution[i]];
        }
        score += self.distance_matrix[self.solution[self.size - 1]][self.solution[0]];

        return score;
    }

    fn reset(&mut self) {
        self.solution = (0..self.size).collect()
    }

    fn set_best(&mut self) {
        self.best_solution = self.solution.to_vec();
    }
}
