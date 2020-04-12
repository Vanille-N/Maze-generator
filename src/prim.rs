use crate::grid::{Direction, Position, Grid, Marker, Wall};
use std::collections::BinaryHeap;
use rand::Rng;
use std::cmp;

#[derive(Eq, PartialEq)]
struct Vertice {
    weight: usize,
    start: Position,
    dir: Direction,
}

pub fn build(g: &mut Grid) {
    let mut rng = rand::thread_rng();
    let mut heap = BinaryHeap::new();
    heap.push(Vertice::new(rng.gen(), Position::new(0, 0), Direction::East));
    heap.push(Vertice::new(rng.gen(), Position::new(0, 0), Direction::South));
    while let Some(Vertice { weight: _, start, dir }) = heap.pop() {
        if g.seek_tolerant(start, dir) && g.get_mark(start.adj(dir)) == Marker::Blank {
            g.open(start, dir);
            let p = start.adj(dir);
            g.mark(p, Marker::Seen);
            for d in Direction::enumerate_rnd() {
                heap.push(Vertice::new(rng.gen(), p.clone(), d));
            }
        }
    }
    g.complete(Wall::Closed);
    g.mark_all(Marker::Blank);
}

impl Vertice {
    fn new(weight: usize, start: Position, dir: Direction) -> Self {
        Vertice { weight, start, dir }
    }
}

impl cmp::Ord for Vertice {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl cmp::PartialOrd for Vertice {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(&other))
    }
}
