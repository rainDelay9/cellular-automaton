pub mod automaton_builder;
pub mod grid;
pub mod neighborhood;
pub mod rules;

use crate::automaton::grid::Grid;
use crate::automaton::rules::rules::Rules;
use crate::utils::coordinates_counter::CoordinatesCounter;
use ndarray::{ArrayD, IxDyn};

#[derive(Debug)]
pub struct Automaton {
    grid: Grid,
    rules: Rules,
    gen: u32,
}

impl Automaton {
    pub fn new(grid: Grid, rules: Rules) -> Self {
        Self {
            grid,
            rules,
            gen: 0,
        }
    }

    pub fn advance_generation(&mut self) {
        let dims = self.grid.dims();
        let cc = CoordinatesCounter::new(&Vec::from(dims));
        let mut new_grid = Grid::new(Vec::from(dims), self.grid.grid());
        for coordinate in cc {
            let neighborhood = self.grid.neighborhood(coordinate.clone());
            match self.rules.apply(&neighborhood) {
                Some(val) => new_grid.set_point(&coordinate[..], val),
                _ => continue,
            }
        }
        self.grid = new_grid.clone();
        self.gen += 1;
    }
}
