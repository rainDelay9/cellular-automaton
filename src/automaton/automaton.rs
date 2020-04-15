use super::space::Space;
use crate::automaton::rules::rules::Rules;

pub struct Automaton {
    space: Space,
    rules: Rules,
}
