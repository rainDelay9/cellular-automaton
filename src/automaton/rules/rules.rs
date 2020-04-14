pub struct Rule<N>
where
    N: std::cmp::Eq,
{
    neighborhood: N,
    result: u32,
}

impl<N> Rule<N>
where
    N: std::cmp::Eq + Copy,
{
    fn apply(&self, _other: N) -> Option<u32> {
        match self.neighborhood {
            _other => Some(self.result),
            _ => None,
        }
    }
}

pub struct Rules<N>
where
    N: std::cmp::Eq + Copy,
{
    rules: Vec<Rule<N>>,
}

impl<N> Rules<N>
where
    N: std::cmp::Eq + Copy,
{
    fn new(rules: Vec<Rule<N>>) -> Self {
        Rules { rules }
    }

    fn analyze(&self, neighborhood: N) -> Option<u32> {
        for rule in &self.rules[..] {
            match rule.apply(neighborhood) {
                Some(val) => return Some(val),
                None => continue,
            }
        }
        None
    }
}
