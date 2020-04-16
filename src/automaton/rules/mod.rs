pub mod parsers;
use crate::automaton::rules::parsers::schemas::ConfigSchema;

#[derive(Debug)]
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
}

impl From<ConfigSchema> for Rules {
    fn from(schema: ConfigSchema) -> Self {
        let mut rules = Vec::new();
        for conf in schema.rules {
            rules.push(Rule::new(conf.neighborhood, conf.cell));
        }
        Self { rules }
    }
}

#[derive(Debug)]
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
}

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
