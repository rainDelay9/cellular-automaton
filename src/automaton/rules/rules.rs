pub struct Rule {
    neighborhood: Vec<u32>,
    result: u32,
}

impl Rule {
    fn apply(&self, _other: &Vec<u32>) -> Option<u32> {
        match &self.neighborhood {
            _other => Some(self.result),
            _ => None,
        }
    }
}

pub struct Rules {
    rules: Vec<Rule>,
}

impl Rules {
    fn new(rules: Vec<Rule>) -> Self {
        Rules { rules }
    }

    fn analyze(&self, neighborhood: &Vec<u32>) -> Option<u32> {
        for rule in &self.rules[..] {
            match rule.apply(neighborhood) {
                Some(val) => return Some(val),
                None => continue,
            }
        }
        None
    }
}
