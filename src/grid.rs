use std::ops;
use std::fmt;
use std::hash::{Hash, Hasher};
use rand::{thread_rng, seq::SliceRandom};


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Position {
    pub i: usize,
    pub j: usize,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Wall {
    Open,
    Closed,
    Uninit,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Marker {
    Blank,
    Seen,
    Done,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

pub struct Room {
    south: Wall,
    east: Wall,
    marker: Marker,
}

pub struct Grid {
    height: usize,
    width: usize,
    layout: Vec<Room>,
}

impl Grid {
    pub fn new(height: usize, width: usize) -> Self {
        let mut layout = Vec::new();
        for _ in 0..height {
            for _ in 0..width {
                layout.push(Room::new());
            }
        }
        for i in 0..height {
            layout[i * width + width-1].east = Wall::Closed;
        }
        for j in 0..width {
            layout[(height-1) * width + j].south = Wall::Closed;
        }
        Grid {
            height,
            width,
            layout,
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn seek(&self, pos: Position, dir: Direction) -> bool {
        let Position { i, j } = pos;
        let (i, j) = (i as isize, j as isize);
        let (i, j) = match dir {
            Direction::North => (i-1, j),
            Direction::South => (i, j),
            Direction::East => (i, j),
            Direction::West => (i, j-1),
        };
        if i >= 0 && j >= 0 && (i as usize) < self.height && (j as usize) < self.width {
            match dir {
                Direction::North | Direction::South => self[[i as usize, j as  usize]].south == Wall::Open,
                Direction::East | Direction::West => self[[i as usize, j as usize]].east == Wall::Open,
            }
        } else {
            false
        }
    }

    pub fn seek_tolerant(&self, pos: Position, dir: Direction) -> bool {
        let Position { i, j } = pos;
        let (i, j) = (i as isize, j as isize);
        let (i, j) = match dir {
            Direction::North => (i-1, j),
            Direction::South => (i, j),
            Direction::East => (i, j),
            Direction::West => (i, j-1),
        };
        if i >= 0 && j >= 0 && (i as usize) < self.height && (j as usize) < self.width {
            match dir {
                Direction::North | Direction::South => self[[i as usize, j as  usize]].south != Wall::Closed,
                Direction::East | Direction::West => self[[i as usize, j as usize]].east != Wall::Closed,
            }
        } else {
            false
        }
    }

    pub fn open(&mut self, pos: Position, dir: Direction) {
        let Position { i, j } = pos;
        let (i, j) = (i as isize, j as isize);
        let (i, j) = match dir {
            Direction::North => (i-1, j),
            Direction::South => (i, j),
            Direction::East => (i, j),
            Direction::West => (i, j-1),
        };
        if i >= 0 && j >= 0 && (i as usize) < self.height && (j as usize) < self.width {
            match dir {
                Direction::North | Direction::South => {
                    self[[i as usize, j as  usize]].south = Wall::Open
                }
                Direction::East | Direction::West => {
                    self[[i as usize, j as usize]].east = Wall::Open
                }
            }
        }
    }

    pub fn close(&mut self, pos: Position, dir: Direction) {
        let Position { i, j } = pos;
        let (i, j) = (i as isize, j as isize);
        let (i, j) = match dir {
            Direction::North => (i-1, j),
            Direction::South => (i, j),
            Direction::East => (i, j),
            Direction::West => (i, j-1),
        };
        if i >= 0 && j >= 0 && (i as usize) < self.height && (j as usize) < self.width {
            match dir {
                Direction::North | Direction::South => {
                    self[[i as usize, j as  usize]].south = Wall::Closed
                }
                Direction::East | Direction::West => {
                    self[[i as usize, j as usize]].east = Wall::Closed
                }
            }
        }
    }

    pub fn complete(&mut self, fill: Wall) {
        for room in self.layout.iter_mut() {
            if room.east == Wall::Uninit { room.east = fill; }
            if room.south == Wall::Uninit { room.south = fill; }
        }
    }

    pub fn mark_all(&mut self, m: Marker) {
        for room in self.layout.iter_mut() {
            room.marker = m;
        }
    }

    pub fn mark(&mut self, p: Position, m: Marker) {
        self[p].marker = m;
    }

    pub fn get_mark(&self, p: Position) -> Marker {
        self[p].marker
    }

    fn calc_lt(&self, i: usize, j: usize) -> bool {
        if i == 0 && j == 1 || i == self.height && j == self.width {
            true
        } else if j == 0 {
            true
        } else if i == 0 || i == self.height {
            false
        } else {
            self.seek_tolerant(Position::new(i, j-1), Direction::North)
        }
    }

    fn calc_rt(&self, i: usize, j: usize) -> bool {
        if i == 0 && j == 0 || i == self.height && j == self.width-1 {
            true
        } else if j == self.width {
            true
        } else if i == 0 || i == self.height {
            false
        } else {
            self.seek_tolerant(Position::new(i, j), Direction::North)
        }
    }

    fn calc_up(&self, i: usize, j: usize) -> bool {
        if i == 0 {
            true
        } else if j == 0 || j == self.width {
            false
        } else {
            self.seek_tolerant(Position::new(i-1, j), Direction::West)
        }
    }

    fn calc_dn(&self, i: usize, j: usize) -> bool {
        if i == self.height {
            true
        } else if j == 0 || j == self.width {
            false
        } else {
            self.seek_tolerant(Position::new(i, j), Direction::West)
        }
    }
}

impl ops::Index<[usize; 2]> for Grid {
    type Output = Room;

    fn index(&self, pos: [usize; 2]) -> &Self::Output {
        &self.layout[pos[0]*self.width + pos[1]]
    }
}

impl ops::IndexMut<[usize; 2]> for Grid {
    fn index_mut(&mut self, pos: [usize; 2]) -> &mut Self::Output {
        &mut self.layout[pos[0] * self.width + pos[1]]
    }
}

impl ops::Index<Position> for Grid {
    type Output = Room;

    fn index(&self, pos: Position) -> &Self::Output {
        &self.layout[pos.i * self.width + pos.j]
    }
}

impl ops::IndexMut<Position> for Grid {
    fn index_mut(&mut self, pos: Position) -> &mut Self::Output {
        &mut self.layout[pos.i * self.width + pos.j]
    }
}

impl Position {
    pub fn new(i: usize, j: usize) -> Self {
        Position {
            i,
            j,
        }
    }

    pub fn mv(&mut self, dir: Direction) {
        match dir {
            Direction::North => self.i -= 1,
            Direction::South => self.i += 1,
            Direction::East => self.j += 1,
            Direction::West => self.j -= 1,
        }
    }

    pub fn adj(&self, dir: Direction) -> Self {
        match dir {
            Direction::North => Position { i: self.i - 1, j: self.j},
            Direction::South => Position { i: self.i + 1, j: self.j},
            Direction::East => Position { i: self.i, j: self.j + 1},
            Direction::West => Position { i: self.i, j: self.j - 1},
        }
    }
}

impl Room {
    pub fn new() -> Self {
        Room {
            south: Wall::Uninit,
            east: Wall::Uninit,
            marker: Marker::Blank,
        }
    }
}

const BOX: [[[[char; 2]; 2]; 2]; 2] = [
    [ // Left on
        [ // Up on
            [ // Right on
                '┼', // Down on
                '┴' // Down off
            ], [ // Right off
                '┤', // Down on
                '┘' // Down off
            ]
        ], [ // Up off
            [ // Right on
                '┬', // Down on
                '─' // Down off
            ], [ // Right off
                '┐', // Down on
                '╴' // Down off
            ]
        ]
    ], [ // Left off
        [ // Up on
            [ // Right on
                '├', // Down on
                '└' // Down off
            ], [ // Right off
                '│', // Down on
                '╵' // Down off
            ]
        ], [ // Up off
            [ // Right on
                '┌', // Down on
                '╶' // Down off
            ], [ // Right off
                '╷', // Down on
                ' ' // Down off
            ]
        ]
    ]
];

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "   ↓")?;
        for i in 0..=self.height {
            write!(f, "  ")?;
            for j in 0..=self.width {
                let key = |b| if b { 1 } else { 0 };
                let lt = self.calc_lt(i, j);
                let up = self.calc_up(i, j);
                let dn = self.calc_dn(i, j);
                let rt = self.calc_rt(i, j);
                write!(f, "{}", BOX[key(lt)][key(up)][key(rt)][key(dn)])?;
                write!(f, "{}", BOX[key(rt)][key(true)][key(rt)][key(true)])?;
            }
            writeln!(f)?;
        }
        for _ in 0..self.width {
            write!(f, "  ")?;
        }
        write!(f, " ↓")
    }
}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.i.hash(state);
        self.j.hash(state);
    }
}

impl Direction {
    pub fn enumerate_rnd() -> std::vec::IntoIter<Direction> {
        let mut v = vec![Self::North, Self::East, Self::South, Self::West];
        v.shuffle(&mut thread_rng());
        v.into_iter()
    }
}
