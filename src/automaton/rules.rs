use crate::automaton::parsers::schemas::RulesSchema;

use exitfailure::ExitFailure;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Rules {
    rules: Vec<Rule>,
}

impl Rules {
    pub fn new(rules: Vec<Rule>) -> Self {
        Rules { rules }
    }

    pub fn apply(&self, neighborhood: &Vec<u32>) -> Option<u32> {
        for rule in &self.rules[..] {
            match rule.apply(neighborhood) {
                Some(val) => return Some(val),
                None => continue,
            }
        }
        None
    }

    pub fn add(&mut self, rules: &mut Vec<Rule>) {
        self.rules.append(rules);
    }

    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    pub fn verify_dimensions(&self, dims: usize) -> Result<(), ExitFailure> {
        for rule in &self.rules {
            if !rule.is_of_dimensions(dims) {
                return Err(ExitFailure::from(DimensionsError::new(
                    "rule has improper dimension",
                )));
            }
        }
        Ok(())
    }
}

impl From<&RulesSchema> for Rules {
    fn from(schema: &RulesSchema) -> Self {
        let mut rules = Vec::new();
        for conf in &schema.rules[..] {
            rules.push(Rule::new(conf.neighborhood.clone(), conf.cell));
        }
        Self { rules }
    }
}

#[derive(Debug, Clone)]
pub struct Rule {
    neighborhood: Vec<u32>,
    result: u32,
}

impl Rule {
    pub fn new(neighborhood: Vec<u32>, result: u32) -> Self {
        Self {
            neighborhood,
            result,
        }
    }

    pub fn apply(&self, _other: &Vec<u32>) -> Option<u32> {
        if &self.neighborhood == _other {
            return Some(self.result);
        }
        None
    }

    pub fn is_of_dimensions(&self, dims: usize) -> bool {
        return self.neighborhood.len() == 3usize.pow(dims as u32);
    }
}

#[derive(Debug)]
struct DimensionsError {
    cause: String,
}

impl DimensionsError {
    pub fn new(cause: &str) -> Self {
        Self {
            cause: cause.to_string(),
        }
    }
}

impl fmt::Display for DimensionsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "dimensions error! {}", self.cause)
    }
}

impl std::error::Error for DimensionsError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_rule() {
        let rule = Rule::new(vec![1, 2, 3], 4);
        let rules_vec = vec![rule];
        let rules = Rules::new(rules_vec);
        match rules.apply(&vec![1, 2, 3]) {
            Some(val) => assert_eq!(val, 4),
            None => assert!(false),
        }
    }
}
