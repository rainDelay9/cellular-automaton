pub struct Neighborhood {
    neighbors: Vec<u32>,
    cell: u32,
}

impl Neighborhood {
    pub fn new(neighbors: Vec<u32>, cell: u32) -> Self {
        Self { neighbors, cell }
    }

    pub fn neighbors(&self) -> Vec<u32> {
        self.neighbors.clone()
    }
    pub fn cell(&self) -> u32 {
        self.cell
    }
}

pub trait Rule {
    fn apply(&self, _neighborhood: &Neighborhood) -> Option<u32>;
}

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

pub struct SumEqualRule {
    rule: SumRule,
}

impl Rule for SumEqualRule {
    fn apply(&self, _neighborhood: &Neighborhood) -> Option<u32> {
        apply_sum_rule_on_predicate(&self.rule, _neighborhood, &|a, b| a == b)
    }
}

pub struct SumLargerRule {
    rule: SumRule,
}

impl Rule for SumLargerRule {
    fn apply(&self, _neighborhood: &Neighborhood) -> Option<u32> {
        apply_sum_rule_on_predicate(&self.rule, _neighborhood, &|a, b| a > b)
    }
}

pub struct SumSmallerRule {
    rule: SumRule,
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
