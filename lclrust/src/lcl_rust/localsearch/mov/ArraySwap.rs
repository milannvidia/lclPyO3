use rand::random;

pub struct ArraySwap{
    pub(crate) size: usize
}

impl ArraySwap{
    pub fn get_random_move(self) -> (u64, u64) {
        let i:u64=random();
        let mut j:u64=random();
        while i==j {
            j=random();
        }
        return (i,j)
    }
}