use std::{any::type_name, fmt::Debug, str::FromStr};

use grid::Grid;
use itertools::Itertools;

// ------------------------------------------------------------------------------------------------
// Data types

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
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

impl Direction {
    pub const fn opposite(self) -> Self {
        match_enum_transform!(self, Direction, [S:N, E:W, N:S, W:E])
    }

    pub const fn opposite_mut(&mut self) {
        *self = self.opposite();
    }

    pub const fn rotate_ccw(self) -> Self {
        match_enum_transform!(self, Direction, [S:E, E:N, N:W, W:S])
    }

    pub const fn rotate_ccw_mut(&mut self) {
        *self = self.rotate_ccw();
    }

    pub const fn rotate_cw(self) -> Self {
        match_enum_transform!(self, Direction, [S:W, E:S, N:E, W:N])
    }

    pub const fn rotate_cw_mut(&mut self) {
        *self = self.rotate_cw();
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub const fn move_dir(&self, dir: Direction) -> Self {
        match dir {
            Direction::S => Self {
                x: self.x + 1,
                y: self.y,
            },
            Direction::E => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::N => Self {
                x: self.x - 1,
                y: self.y,
            },
            Direction::W => Self {
                x: self.x,
                y: self.y - 1,
            },
        }
    }

    pub fn move_dir_mut(&mut self, dir: Direction) {
        match dir {
            Direction::S => self.x += 1,
            Direction::E => self.y += 1,
            Direction::N => self.x -= 1,
            Direction::W => self.y -= 1,
        }
    }

    #[allow(clippy::cast_possible_wrap)]
    #[allow(clippy::cast_sign_loss)]
    pub const fn manhattan_distance(&self, other: Self) -> u64 {
        ((self.x as i64 - other.x as i64).abs() + (self.y as i64 - other.y as i64).abs()) as u64
    }
}

impl From<(usize, usize)> for Pos {
    fn from(value: (usize, usize)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<Pos> for (usize, usize) {
    fn from(val: Pos) -> Self {
        (val.x, val.y)
    }
}

pub trait PosGet {
    fn pos_get(&self, pos: Pos) -> Option<&u8>;

    fn pos_get_mut(&mut self, pos: Pos) -> Option<&mut u8>;
}

impl PosGet for Grid<u8> {
    fn pos_get(&self, pos: Pos) -> Option<&u8> {
        self.get(pos.x, pos.y)
    }

    fn pos_get_mut(&mut self, pos: Pos) -> Option<&mut u8> {
        self.get_mut(pos.x, pos.y)
    }
}

// ------------------------------------------------------------------------------------------------
// Functions

pub fn parse_expect<F>(string: &str) -> F
where
    F: FromStr,
    <F as FromStr>::Err: Debug,
{
    string
        .parse::<F>()
        .unwrap_or_else(|_| panic!("Expected {}", type_name::<F>()))
}

// ------------------------------------------------------------------------------------------------
// Parsers

pub fn bytes_grid(input: &str) -> Grid<u8> {
    Grid::from(
        input
            .lines()
            .map(|line| line.bytes().collect())
            .collect_vec(),
    )
}
