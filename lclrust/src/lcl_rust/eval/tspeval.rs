pub struct Tspeval {
    pub(crate) distance_matrix:Vec<Vec<u64>>,
    pub(crate) size:u64
}

impl Tspeval {
    pub fn evaluate(self,order:Vec<usize>)->u64{
        let mut score:u64=0;
        for i in 1..self.size {
            score+=self.distance_matrix[order[(i-1) as usize]][order[i as usize]];
        }
        return score;
    }
}