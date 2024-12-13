use std::ops::{Index, IndexMut};

use super::pos::Pos;

pub struct GridMask {
    pub mask: Vec<bool>,
    pub cols: usize,
}

impl GridMask {
    pub fn new((rows, cols): (usize, usize)) -> Self {
        Self {
            mask: vec![false; rows * cols],
            cols,
        }
    }

    #[allow(clippy::cast_sign_loss)]
    pub fn set_true(&mut self, pos: Pos) -> bool {
        if self[pos] {
            false
        } else {
            self[pos] = true;
            true
        }
    }
}

#[allow(clippy::cast_sign_loss)]
impl Index<Pos> for GridMask {
    type Output = bool;

    fn index(&self, pos: Pos) -> &Self::Output {
        &self.mask[pos.x as usize * self.cols + pos.y as usize]
    }
}

#[allow(clippy::cast_sign_loss)]
impl IndexMut<Pos> for GridMask {
    fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
        &mut self.mask[pos.x as usize * self.cols + pos.y as usize]
    }
}
