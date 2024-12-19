use grid::Grid;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Dir {
    S,
    E,
    N,
    W,
}

macro_rules! match_enum_transform {
    ($match:expr, $enum:ident, [$($v1:ident : $v2:ident),+]) => {
        match $match {
            $( $enum::$v1 => $enum::$v2,)+
        }
    };
}

impl Dir {
    pub const fn opposite(self) -> Self {
        match_enum_transform!(self, Dir, [S:N, E:W, N:S, W:E])
    }

    pub const fn opposite_mut(&mut self) {
        *self = self.opposite();
    }

    pub const fn rotate_ccw(self) -> Self {
        match_enum_transform!(self, Dir, [S:E, E:N, N:W, W:S])
    }

    pub const fn rotate_ccw_mut(&mut self) {
        *self = self.rotate_ccw();
    }

    pub const fn rotate_cw(self) -> Self {
        match_enum_transform!(self, Dir, [S:W, E:S, N:E, W:N])
    }

    pub const fn rotate_cw_mut(&mut self) {
        *self = self.rotate_cw();
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

impl Pos {
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub const fn move_dir(&self, dir: Dir) -> Self {
        match dir {
            Dir::S => Self::new(self.x + 1, self.y),
            Dir::E => Self::new(self.x, self.y + 1),
            Dir::N => Self::new(self.x - 1, self.y),
            Dir::W => Self::new(self.x, self.y - 1),
        }
    }

    pub const fn move_dir_mut(&mut self, dir: Dir) {
        match dir {
            Dir::S => self.x += 1,
            Dir::E => self.y += 1,
            Dir::N => self.x -= 1,
            Dir::W => self.y -= 1,
        }
    }

    pub const fn in_bounds(&self, bounds: (Self, Self)) -> bool {
        self.x >= bounds.0.x && self.x < bounds.1.x && self.y >= bounds.0.y && self.y < bounds.1.y
    }

    pub const fn manhattan_distance(&self, other: Self) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn neighbors(&self) -> impl Iterator<Item = Self> {
        [
            Self::new(self.x + 1, self.y),
            Self::new(self.x, self.y + 1),
            Self::new(self.x - 1, self.y),
            Self::new(self.x, self.y - 1),
        ]
        .into_iter()
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Pos {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl From<(isize, isize)> for Pos {
    fn from(value: (isize, isize)) -> Self {
        Self::new(value.0, value.1)
    }
}

#[allow(clippy::cast_possible_wrap)]
impl From<(usize, usize)> for Pos {
    fn from(value: (usize, usize)) -> Self {
        Self::new(value.0 as isize, value.1 as isize)
    }
}

#[allow(clippy::cast_sign_loss)]
impl From<Pos> for (usize, usize) {
    fn from(val: Pos) -> Self {
        (val.x as usize, val.y as usize)
    }
}

pub trait GridPosGet {
    fn pos_get(&self, pos: Pos) -> Option<&u8>;

    fn pos_get_expect(&self, pos: Pos) -> &u8;

    fn pos_get_mut_expect(&mut self, pos: Pos) -> &mut u8;
}

impl GridPosGet for Grid<u8> {
    fn pos_get(&self, pos: Pos) -> Option<&u8> {
        self.get(pos.x, pos.y)
    }

    fn pos_get_expect(&self, pos: Pos) -> &u8 {
        self.get(pos.x, pos.y).expect("Expected byte")
    }

    fn pos_get_mut_expect(&mut self, pos: Pos) -> &mut u8 {
        self.get_mut(pos.x, pos.y).expect("Expected byte")
    }
}
