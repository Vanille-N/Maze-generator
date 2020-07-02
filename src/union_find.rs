use std::collections::HashMap;
use std::hash::Hash;

pub struct Union<T: Eq + Hash> {
    id: HashMap<T, usize>,
    sets: Vec<usize>,
}

impl<T: Eq + Hash> Union<T> {
    pub fn new() -> Self {
        Union {
            id: HashMap::new(),
            sets: Vec::new(),
        }
    }

    pub fn insert(&mut self, elem: T) {
        let n = self.sets.len();
        self.id.insert(elem, n);
        self.sets.push(n);
    }

    fn find(&mut self, elem: T) -> Option<usize> {
        let id = self.id.get(&elem).map(|n| *n);
        match id {
            Some(n) => Some(self.find_helper(n)),
            None => None,
        }
    }

    pub fn get_class(&mut self, elem: T) -> usize {
        self.find(elem).unwrap()
    }

    fn find_helper(&mut self, elem: usize) -> usize {
        if elem != self.sets[elem] {
            self.sets[elem] = self.find_helper(self.sets[elem]);
        }
        self.sets[elem]
    }

    pub fn are_joined(&mut self, a: T, b: T) -> bool {
        self.find(a) == self.find(b)
    }

    pub fn join(&mut self, a: T, b: T) {
        if let Some(a) = self.find(a) {
            if let Some(b) = self.find(b) {
                self.sets[a] = b;
            }
        }
    }
}
