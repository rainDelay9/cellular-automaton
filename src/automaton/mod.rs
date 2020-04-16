pub mod automaton_builder;
pub mod grid;
pub mod neighborhood;
pub mod rules;

use crate::automaton::grid::Grid;
use crate::automaton::rules::Rules;
use crate::utils::coordinates_iterator::CoordinatesIterator;

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
        let ci = CoordinatesIterator::new(&Vec::from(dims));
        let mut new_grid = Grid::new(Vec::from(dims), self.grid.grid());
        for coordinate in ci {
            let neighborhood = self.grid.neighborhood(coordinate.clone());
            match self.rules.apply(&neighborhood) {
                Some(val) => new_grid.set_point(&coordinate[..], val),
                _ => continue,
            }
        }
        self.grid = new_grid.clone();
        self.gen += 1;
    }

    pub fn advance_multi(&mut self, gens: u32) {
        for _ in 0..gens {
            self.advance();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::automaton::grid::Grid;
    use crate::automaton::rules::{Rule, Rules};
    use ndarray::{ArrayD, IxDyn};
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

        let mut automaton = Automaton::new(Grid::new(dims, grid.clone()), rules);
        assert_eq!(grid, automaton.grid.grid());
        automaton.advance();
        let g = automaton.grid.grid();
        assert_eq!(g.as_slice().unwrap(), [1, 4, 3]);
    }
    #[test]
    fn test_rule_110() {
        let rules_vec = vec![
            Rule::new(vec![0, 0, 0], 0),
            Rule::new(vec![0, 0, 1], 1),
            Rule::new(vec![0, 1, 0], 1),
            Rule::new(vec![0, 1, 1], 1),
            Rule::new(vec![1, 0, 0], 0),
            Rule::new(vec![1, 0, 1], 1),
            Rule::new(vec![1, 1, 0], 1),
            Rule::new(vec![1, 1, 1], 0),
        ];
        let rules = Rules::new(rules_vec);
        let dims = vec![26];
        let mut grid: ArrayD<u32> = ArrayD::zeros(IxDyn(&dims[..]));
        grid[[5]] = 1;
        grid[[8]] = 1;
        grid[[12]] = 1;
        grid[[18]] = 1;

        let mut automaton = Automaton::new(Grid::new(dims, grid.clone()), rules);
        assert_eq!(grid, automaton.grid.grid());
        automaton.advance();
        let g = automaton.grid.grid();
        assert_eq!(
            g.as_slice().unwrap(),
            [0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn test_rule_110_1000_generations() {
        let rules_vec = vec![
            Rule::new(vec![0, 0, 0], 0),
            Rule::new(vec![0, 0, 1], 1),
            Rule::new(vec![0, 1, 0], 1),
            Rule::new(vec![0, 1, 1], 1),
            Rule::new(vec![1, 0, 0], 0),
            Rule::new(vec![1, 0, 1], 1),
            Rule::new(vec![1, 1, 0], 1),
            Rule::new(vec![1, 1, 1], 0),
        ];
        let rules = Rules::new(rules_vec);
        let dims = vec![26];
        let mut grid: ArrayD<u32> = ArrayD::zeros(IxDyn(&dims[..]));
        grid[[5]] = 1;
        grid[[8]] = 1;
        grid[[12]] = 1;
        grid[[18]] = 1;

        let mut automaton = Automaton::new(Grid::new(dims, grid.clone()), rules);
        assert_eq!(grid, automaton.grid.grid());
        automaton.advance_multi(1000);
        let g = automaton.grid.grid();
        assert_eq!(
            g.as_slice().unwrap(),
            [1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 1, 1, 1, 0, 0, 0, 1, 0, 0]
        );
    }
}
