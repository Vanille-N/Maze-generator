use rand::{thread_rng, Rng, seq::SliceRandom};
use std::collections::HashSet;
use crate::union_find::Union;
use crate::grid::{Direction, Position, Grid, Wall};

pub fn build(g: &mut Grid) {
    let mut union = Union::<Position>::new();
    for i in 0..g.height() {
        for j in 0..g.width() {
            let p = Position::new(i, j);
            union.insert(p);
        }
    }
    let mut rng = thread_rng();
    for i in 0..g.height()-1 {
        for j in 0..g.width()-1 {
            let curr = Position::new(i, j);
            let neigh = Position::new(i, j+1);
            if !union.are_joined(curr, neigh) && rng.gen::<f64>() < 0.5 {
                g.open(curr, Direction::East);
                union.join(curr, neigh);
            }
        }
        let mut paths = Vec::new();
        for j in 0..g.width() {
            let p = Position::new(i, j);
            paths.push((p, union.get_class(p)));
        }
        for p in choose_in_each(paths) {
            g.open(p, Direction::South);
            union.join(p, p.adj(Direction::South));
        }
    }
    for j in 0..g.width()-1 {
        let curr = Position::new(g.height()-1, j);
        let neigh = Position::new(g.width()-1, j+1);
        if !union.are_joined(curr, neigh) {
            g.open(curr, Direction::East);
            union.join(curr, neigh);
        }
    }
    g.complete(Wall::Closed);
}

fn choose_in_each(mut v: Vec<(Position, usize)>) -> Vec<Position> {
    let mut rng = thread_rng();
    v.shuffle(&mut rng);
    v.sort_by_key(|e| e.1);
    let mut s = HashSet::new();
    let mut select = Vec::new();
    for (p, i) in v {
        if !s.contains(&i) || rng.gen::<f64>() < 0.2 {
            s.insert(i);
            select.push(p);
        }
    }
    select
}
