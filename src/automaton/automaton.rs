use super::space::Space;
use crate::automaton::rules::rules::Rules;

pub struct Automaton {
    space: Space,
    rules: Rules,
}

impl Automaton {
    pub fn new(space: Space, rules: Rules) -> Self {
        Self { space, rules }
    }
}
