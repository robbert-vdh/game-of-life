use nalgebra::DMatrix;
use std::ops::{Index, IndexMut};

/// The game's grid, internally represented by matrices provided by the nalgebra crate.
pub struct Grid {
    matrix: DMatrix<bool>,
}

impl Grid {
    /// Create a new grid with the specified dimensions. All cells default to `false`.
    pub fn new(rows: usize, columns: usize) -> Grid {
        Grid { matrix: DMatrix::from_element(rows, columns, false) }
    }

    /// The number of columns in the grid.
    pub fn cols(&self) -> usize {
        self.matrix.ncols()
    }

    /// The number of rows in the grid.
    pub fn rows(&self) -> usize {
        self.matrix.nrows()
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = bool;

    fn index(&self, index: (usize, usize)) -> &bool {
        &self.matrix[index]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut bool {
        &mut self.matrix[index]
    }
}
