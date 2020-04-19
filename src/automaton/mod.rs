pub mod automaton_builder;
pub mod grid;
pub mod neighborhood;
pub mod parsers;
pub mod rules;

use std::fmt;

use crate::automaton::grid::Grid;
use crate::automaton::rules::Rules;
use crate::utils::coordinates_iterator::CoordinatesIterator;
use exitfailure::ExitFailure;

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

    pub fn advance(&mut self) -> Result<(), ExitFailure> {
        let dims = self.grid.dims();
        let ci = CoordinatesIterator::new(&Vec::from(dims));
        let mut new_grid = Grid::new(Vec::from(dims), self.grid.grid());
        for coordinate in ci {
            let neighborhood = self.grid.neighborhood(coordinate.clone())?;
            match self.rules.apply(&neighborhood) {
                Some(val) => new_grid.set_point(&coordinate[..], val)?,
                _ => continue,
            }
        }
        self.grid = new_grid.clone();
        self.gen += 1;
        Ok(())
    }

    pub fn advance_multi(&mut self, gens: u32) -> Result<(), ExitFailure> {
        for _ in 0..gens {
            self.advance()?;
        }
        Ok(())
    }

    pub fn set_point(&mut self, point: &[usize]) -> Result<(), ExitFailure> {
        self.grid.set_point(point, 1)?;
        Ok(())
    }
}

impl fmt::Display for Automaton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "generation {}\ngrid:\n {}", self.gen, self.grid)
    }
}
