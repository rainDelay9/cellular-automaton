pub mod automaton_builder;
pub mod grid;
pub mod neighborhood;
pub mod rules;

use crate::automaton::grid::Grid;
use crate::automaton::rules::Rules;
use crate::utils::coordinates_counter::CoordinatesCounter;
use ndarray::{array, ArrayD, IxDyn};

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

    pub fn advance(&mut self) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::automaton::grid::Grid;
    use crate::automaton::rules::{Rule, Rules};
    #[test]
    fn test_advance_generation() {
        let rule = Rule::new(vec![1, 2, 3], 4);
        let rules_vec = vec![rule];
        let rules = Rules::new(rules_vec);
        let dims = vec![3];
        let mut grid: ArrayD<u32> = ArrayD::zeros(IxDyn(&dims[..]));
        grid[[0]] = 1;
        grid[[1]] = 2;
        grid[[2]] = 3;

        let mut automaton = Automaton::new(Grid::new(dims, grid), rules);
        println!("grid 1: {:?}", automaton.grid.grid());
        automaton.advance();
        println!("grid 2: {:?}", automaton.grid.grid());
    }
}
