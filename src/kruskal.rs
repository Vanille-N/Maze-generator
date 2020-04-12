use std::collections::HashMap;
use rand::{thread_rng, seq::SliceRandom};

use crate::grid::{Direction, Position, Grid};

struct Union {
    id: HashMap<Position, usize>,
    sets: Vec<usize>,
}

pub fn build(g: &mut Grid) {
    let mut union = Union::new();
    let mut walls = Vec::new();
    for i in 0..g.height() {
        for j in 0..g.width() {
            let p = Position::new(i, j);
            union.insert(p);
            if j != g.width()-1 {
                walls.push((p, Direction::East));
            }
            if i != g.height()-1 {
                walls.push((p, Direction::South));
            }
        }
    }
    walls.shuffle(&mut thread_rng());
    for (p, d) in walls {
        if union.are_joined(p, p.adj(d)) {
            g.close(p, d);
        } else {
            g.open(p, d);
            union.join(p, p.adj(d));
        }
    }
}

impl Union {
    fn new() -> Self {
        Union {
            id: HashMap::new(),
            sets: Vec::new(),
        }
    }

    fn insert(&mut self, elem: Position) {
        let n = self.sets.len();
        self.id.insert(elem, n);
        self.sets.push(n);
    }

    fn find(&mut self, elem: Position) -> Option<usize> {
        let id = self.id.get(&elem).map(|n| *n);
        match id {
            Some(n) => Some(self.find_helper(n)),
            None => None,
        }
    }

    fn find_helper(&mut self, elem: usize) -> usize {
        if elem != self.sets[elem] {
            self.sets[elem] = self.find_helper(self.sets[elem]);
        }
        self.sets[elem]
    }

    fn are_joined(&mut self, a: Position, b: Position) -> bool {
        self.find(a) == self.find(b)
    }

    fn join(&mut self, a: Position, b: Position) {
        if let Some(a) = self.find(a) {
            if let Some(b) = self.find(b) {
                self.sets[a] = b;
            }
        }
    }
}
