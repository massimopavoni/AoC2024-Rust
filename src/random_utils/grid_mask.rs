use std::ops::{Index, IndexMut};

use super::pos::Pos;

#[derive(Debug)]
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

    pub fn set_true(&mut self, pos: Pos) -> bool {
        if self[pos] {
            false
        } else {
            self[pos] = true;
            true
        }
    }
}

impl Index<Pos> for GridMask {
    type Output = bool;

    fn index(&self, pos: Pos) -> &Self::Output {
        #[allow(clippy::cast_sign_loss)]
        &self.mask[pos.x as usize * self.cols + pos.y as usize]
    }
}

impl IndexMut<Pos> for GridMask {
    fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
        #[allow(clippy::cast_sign_loss)]
        &mut self.mask[pos.x as usize * self.cols + pos.y as usize]
    }
}
