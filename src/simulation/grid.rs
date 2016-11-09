use nalgebra::DMatrix;
use std::cmp::{min, max};
use std::ops::{Index, IndexMut};

/// The game's grid, internally represented by matrices provided by the nalgebra crate.
#[derive(Clone)]
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

    pub fn neighbours(&self, (x, y): (usize, usize)) -> u8 {
        let mut total = 0;

        for p in max(x, 1) - 1..min(x + 2, self.rows() - 1) {
            for q in max(y, 1) - 1..min(y + 2, self.cols() - 1) {
                if self[(p, q)] {
                    total += 1;
                }
            }
        }

        total
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounds_are_reported_correctly() {
        let grid = Grid::new(10, 10);

        assert_eq!(10, grid.rows());
        assert_eq!(10, grid.cols());

        assert_eq!(false, grid[(9, 9)]);
    }

    #[test]
    #[should_panic]
    fn horizontal_bound_should_overflow() {
        let grid = Grid::new(10, 10);

        grid[(10, 9)];
    }

    #[test]
    #[should_panic]
    fn vertical_bound_should_overflow() {
        let grid = Grid::new(10, 10);

        grid[(9, 10)];
    }

    #[test]
    fn indexing_works() {
        let mut grid = Grid::new(10, 10);

        for i in 0..10 {
            grid[(i, 0)] = true;
        }

        for x in 0..10 {
            for y in 0..10 {
                if y == 0 {
                    assert_eq!(true, grid[(x, y)])
                } else {
                    assert_eq!(false, grid[(x, y)])
                }
            }
        }
    }

    #[test]
    fn neighbours_are_reported_correctly() {
        let mut grid = Grid::new(10, 10);
        grid[(4, 4)] = true;
        grid[(5, 4)] = true;
        grid[(6, 4)] = true;
        grid[(4, 6)] = true;

        assert_eq!(4, grid.neighbours((5, 5)));
    }

    #[test]
    fn neighbours_are_reported_correctly_for_corners() {
        let mut grid = Grid::new(10, 10);
        grid[(1, 0)] = true;
        grid[(1, 1)] = true;
        grid[(0, 1)] = true;

        assert_eq!(3, grid.neighbours((0, 0)));
    }
}
