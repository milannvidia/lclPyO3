pub(crate) struct ArrayProblem {
    pub(crate) evaluationFunc:fn(),
    pub(crate) moveFunc:fn(),
    pub(crate) order:Vec<u64>
}
impl ArrayProblem{
    fn mov(self){
        self.moveFunc;
    }
}