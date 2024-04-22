pub struct TabuList<T> {
    pub tabu_list: Vec<T>,
}
impl<T: PartialEq> TabuList<T> {
    pub fn new() -> Self {
        TabuList { tabu_list: vec![] }
    }
    pub fn add(&mut self, object: T) {
        self.tabu_list.push(object);
    }
    pub fn remove_first(&mut self) {
        self.tabu_list.pop();
    }
    pub fn contains(&self, object: &T) -> bool {
        return self.tabu_list.contains(object);
    }
}
