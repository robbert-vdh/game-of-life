//! Implemetns Conway's Game of Life for a data structure.

use super::Grid;

pub trait GameOfLife {
    /// Run a single cycle of the simulation.
    fn simulate(&self) -> Self;
}

impl GameOfLife for Grid {
    fn simulate(&self) -> Grid {
        let mut result = self.clone();

        for x in 0..self.cols() {
            for y in 0..self.rows() {
                match self.neighbours((x, y)) {
                    0...1 | 4...9 => result[(x, y)] = false,
                    3 => result[(x, y)] = true,
                    _ => (),
                }
            }
        }

        result
    }
}
