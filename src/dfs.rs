use crate::grid::{Direction, Position, Grid, Marker, Wall};

pub fn build(g: &mut Grid) {
    let mut stack = Vec::new();
    stack.push((Direction::enumerate_rnd(), Position::new(0, 0)));
    g.mark(Position::new(0, 0), Marker::Seen);
    while let Some((mut dir, pos)) = stack.pop() {
        let d = dir.next();
        stack.push((dir, pos));
        if let Some(d) = d {
            if g.seek_tolerant(pos, d) && g.get_mark(pos.adj(d)) == Marker::Blank {
                g.mark(pos.adj(d), Marker::Seen);
                g.open(pos, d);
                stack.push((Direction::enumerate_rnd(), pos.adj(d)));
            }
        } else {
            stack.pop();
            g.mark(pos, Marker::Done);
        }
    }
    g.mark_all(Marker::Blank);
    g.complete(Wall::Closed);
}
