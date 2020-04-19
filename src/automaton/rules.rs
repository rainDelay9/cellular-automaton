use crate::automaton::neighborhood::Neighborhood;
use crate::automaton::parsers::schemas::{RulesSchema, SumRuleSchema};

use dyn_clone::DynClone;

#[derive(Clone)]
pub struct Rules {
    rules: Vec<Box<dyn Rule>>,
}

impl Rules {
    pub fn new(rules: Vec<Box<dyn Rule>>) -> Self {
        Rules { rules }
    }

    pub fn rules(self) -> Vec<Box<dyn Rule>> {
        self.rules
    }

    pub fn apply(&self, neighborhood: &Neighborhood) -> Option<u32> {
        for rule in &self.rules[..] {
            match rule.apply(neighborhood) {
                Some(val) => return Some(val),
                None => continue,
            }
        }
        None
    }

    pub fn add(&mut self, rules: &mut Vec<Box<dyn Rule>>) {
        self.rules.append(rules);
    }

    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}

impl From<&RulesSchema> for Rules {
    fn from(schema: &RulesSchema) -> Self {
        let mut rules = Vec::new();
        for conf in &schema.sum_rules[..] {
            rules.push(parse_rule_from_schema(conf));
        }
        for conf in &schema.explicit_rules[..] {
            rules.push(Box::new(ExplicitRule::new(
                conf.neighborhood.clone(),
                conf.current,
                conf.next,
            )));
        }
        Self { rules }
    }
}

pub fn parse_rule_from_schema(schema: &SumRuleSchema) -> Box<dyn Rule> {
    let sr = SumRule::new(schema.neighborhood, schema.current, schema.next);
    match schema.rule_type {
        0 => return Box::new(SumEqualRule::new(sr)),
        1 => return Box::new(SumLargerRule::new(sr)),
        2 => return Box::new(SumSmallerRule::new(sr)),
        _ => panic!("unsupported rule type!"),
    }
}

pub trait Rule: DynClone {
    fn apply(&self, _neighborhood: &Neighborhood) -> Option<u32>;
}

dyn_clone::clone_trait_object!(Rule);

#[derive(Clone)]
pub struct ExplicitRule {
    neighborhood: Vec<u32>,
    current: u32,
    next: u32,
}

impl Rule for ExplicitRule {
    fn apply(&self, neighborhood: &Neighborhood) -> Option<u32> {
        if &self.neighborhood == &neighborhood.neighbors() && self.current == neighborhood.cell() {
            return Some(self.next);
        }
        None
    }
}

impl ExplicitRule {
    pub fn new(neighborhood: Vec<u32>, current: u32, next: u32) -> Self {
        Self {
            neighborhood,
            current,
            next,
        }
    }
}

#[derive(Clone)]
pub struct SumRule {
    neighborhood: u32,
    current: u32,
    next: u32,
}

impl SumRule {
    pub fn new(neighborhood: u32, current: u32, next: u32) -> Self {
        Self {
            neighborhood,
            current,
            next,
        }
    }

    pub fn neighborhood(&self) -> u32 {
        self.neighborhood
    }
    pub fn current(&self) -> u32 {
        self.current
    }
    pub fn next(&self) -> u32 {
        self.next
    }
}

#[derive(Clone)]
pub struct SumEqualRule {
    rule: SumRule,
}

impl SumEqualRule {
    pub fn new(structure: SumRule) -> Self {
        Self { rule: structure }
    }
}

impl Rule for SumEqualRule {
    fn apply(&self, _neighborhood: &Neighborhood) -> Option<u32> {
        apply_sum_rule_on_predicate(&self.rule, _neighborhood, &|a, b| a == b)
    }
}

#[derive(Clone)]
pub struct SumLargerRule {
    rule: SumRule,
}

impl SumLargerRule {
    pub fn new(structure: SumRule) -> Self {
        Self { rule: structure }
    }
}

impl Rule for SumLargerRule {
    fn apply(&self, _neighborhood: &Neighborhood) -> Option<u32> {
        apply_sum_rule_on_predicate(&self.rule, _neighborhood, &|a, b| a > b)
    }
}

#[derive(Clone)]
pub struct SumSmallerRule {
    rule: SumRule,
}

impl SumSmallerRule {
    pub fn new(structure: SumRule) -> Self {
        Self { rule: structure }
    }
}

impl Rule for SumSmallerRule {
    fn apply(&self, _neighborhood: &Neighborhood) -> Option<u32> {
        apply_sum_rule_on_predicate(&self.rule, _neighborhood, &|a, b| a < b)
    }
}

fn apply_sum_rule_on_predicate(
    sum_rule: &SumRule,
    neighborhood: &Neighborhood,
    pred: &dyn Fn(u32, u32) -> bool,
) -> Option<u32> {
    if pred(
        neighborhood.neighbors().iter().sum(),
        sum_rule.neighborhood(),
    ) && sum_rule.current() == neighborhood.cell()
    {
        return Some(sum_rule.next());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_change_on_equal() {
        let neighborhood = Neighborhood::new(vec![1, 1, 1, 0, 1, 0], 1);
        let ser = SumEqualRule {
            rule: SumRule::new(4, 1, 0),
        };
        let res = ser.apply(&neighborhood);
        assert!(res.is_some());
        assert_eq!(res.unwrap(), 0);
    }

    #[test]
    fn should_change_on_larger() {
        let neighborhood = Neighborhood::new(vec![1, 1, 1, 0, 1, 0], 1);
        let slr = SumLargerRule {
            rule: SumRule::new(3, 1, 0),
        };
        let res = slr.apply(&neighborhood);
        assert!(res.is_some());
        assert_eq!(res.unwrap(), 0);
    }

    #[test]
    fn should_change_on_smaller() {
        let neighborhood = Neighborhood::new(vec![1, 1, 1, 0, 1, 0], 1);
        let ssr = SumSmallerRule {
            rule: SumRule::new(5, 1, 0),
        };
        let res = ssr.apply(&neighborhood);
        assert!(res.is_some());
        assert_eq!(res.unwrap(), 0);
    }

    #[test]
    fn should_not_change_on_equal() {
        let neighborhood = Neighborhood::new(vec![1, 1, 1, 0, 1, 1], 1);
        let ser = SumEqualRule {
            rule: SumRule::new(4, 1, 0),
        };
        let res = ser.apply(&neighborhood);
        assert!(!res.is_some());
    }

    #[test]
    fn should_not_change_on_larger() {
        let neighborhood = Neighborhood::new(vec![1, 1, 1, 0, 1, 0], 1);
        let slr = SumLargerRule {
            rule: SumRule::new(4, 1, 0),
        };
        let res = slr.apply(&neighborhood);
        assert!(!res.is_some());
    }
    #[test]
    fn should_not_change_on_smaller() {
        let neighborhood = Neighborhood::new(vec![1, 1, 1, 0, 1, 1], 1);
        let ssr = SumSmallerRule {
            rule: SumRule::new(5, 1, 0),
        };
        let res = ssr.apply(&neighborhood);
        assert!(!res.is_some());
    }
}
