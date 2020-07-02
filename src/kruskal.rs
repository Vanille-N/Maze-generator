use rand::{thread_rng, seq::SliceRandom};
use crate::union_find::Union;
use crate::grid::{Direction, Position, Grid};

pub fn build(g: &mut Grid) {
    let mut union = Union::<Position>::new();
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
