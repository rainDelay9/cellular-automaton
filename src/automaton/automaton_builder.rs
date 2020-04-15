use crate::automaton::automaton::Automaton;
use crate::automaton::rules::rules::Rules;
use crate::automaton::space::Space;
use exitfailure::ExitFailure;
use ndarray::{ArrayD, IxDyn};
use std::fmt;

pub struct AutomatonBuilder {
    space: Space,
    rules: Option<Rules>,
}

impl AutomatonBuilder {
    /// create a new builder with given dimensions
    pub fn new(dims: Vec<usize>) -> Self {
        Self {
            space: Space::new(dims.clone(), ArrayD::zeros(IxDyn(&dims[..]))),
            rules: None,
        }
    }

    /// generate a new EncryptedBox
    pub fn build(self) -> Result<Automaton, ExitFailure> {
        if !Option::is_some(&self.rules) {
            return Err(ExitFailure::from(BuildError::new(
                "cannot build an automaton with no rules",
            )));
        }
        Ok(Automaton::new(self.space, self.rules.unwrap()))
    }

    /// add a field to the encryption data
    pub fn set_point<'a>(&'a mut self, point: &[usize]) -> &'a mut Self {
        self.space.set_point(point);
        self
    }

    /// add multiple fields at once
    pub fn set_points<'a>(&'a mut self, points: &[&[usize]]) -> &'a mut Self {
        for point in points {
            self.set_point(point);
        }
        self
    }

    /// set a password (of which a key will be derived)
    pub fn set_rules<'a>(&'a mut self, rules: Rules) -> &'a mut Self {
        self.rules = Some(rules);
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
