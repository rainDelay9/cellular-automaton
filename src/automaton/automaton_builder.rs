use crate::automaton::grid::Grid;
use crate::automaton::rules::{Rule, Rules};
use crate::automaton::Automaton;
use exitfailure::ExitFailure;
use ndarray::{ArrayD, IxDyn};
use std::fmt;

pub struct AutomatonBuilder {
    grid: Grid,
    rules: Rules,
}

impl AutomatonBuilder {
    /// create a new builder with given dimensions
    pub fn new(dims: Vec<usize>) -> AutomatonBuilder {
        AutomatonBuilder {
            grid: Grid::new(dims.clone(), ArrayD::zeros(IxDyn(&dims[..]))),
            rules: Rules::new(Vec::new()),
        }
    }

    /// generate a new Automaton
    pub fn build(&mut self) -> Result<Automaton, ExitFailure> {
        if self.rules.is_empty() {
            return Err(ExitFailure::from(BuildError::new(
                "cannot build an automaton with no rules",
            )));
        }
        Ok(Automaton::new(self.grid.clone(), self.rules.clone()))
    }

    /// add a field to the encryption data
    pub fn set_point<'a>(&'a mut self, point: &[usize]) -> Result<&'a mut Self, ExitFailure> {
        self.grid.set_point(point, 1)?;
        Ok(self)
    }

    /// set multiple points at once
    pub fn set_points<'a>(&'a mut self, points: &[&[usize]]) -> Result<&'a mut Self, ExitFailure> {
        for point in points {
            self.set_point(point)?;
        }
        Ok(self)
    }

    /// add rule
    pub fn add_rule<'a>(&'a mut self, rule: Rule) -> &'a mut Self {
        self.rules.add(&mut vec![rule]);
        self
    }

    /// add rules
    pub fn add_rules<'a>(&'a mut self, rules: &mut Vec<Rule>) -> &'a mut Self {
        self.rules.add(rules);
        self
    }

    /// set rules
    pub fn set_rules<'a>(&'a mut self, rules: Rules) -> &'a mut AutomatonBuilder {
        self.rules = rules;
        self
    }
}

#[derive(Debug)]
struct BuildError {
    cause: String,
}

impl BuildError {
    pub fn new(cause: &str) -> BuildError {
        BuildError {
            cause: cause.to_string(),
        }
    }
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "build failed! {}", self.cause)
    }
}

impl std::error::Error for BuildError {}
