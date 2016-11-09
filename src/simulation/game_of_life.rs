//! Implemetns Conway's Game of Life for a data structure.

use super::Grid;

pub trait GameOfLife {
    /// Run a single cycle of the simulation.
    fn simulate(&self) -> Self;
}

impl GameOfLife for Grid {
    fn simulate(&self) -> Grid {
        let mut result = self.clone();

        for x in 0..self.rows() {
            for y in 0..self.cols() {
                match self.neighbours((x, y)) {
                    0...1 | 3 => result[(x, y)] = false,
                    4 => result[(x, y)] = true,
                    _ => (),
                }
            }
        }

        result
    }
}
