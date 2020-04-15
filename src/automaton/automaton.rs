use super::grid::Grid;
use crate::automaton::rules::rules::Rules;

pub struct Automaton {
    grid: Grid,
    rules: Rules,
}

impl Automaton {
    pub fn new(grid: Grid, rules: Rules) -> Self {
        Self { grid, rules }
    }
}
