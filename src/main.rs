pub use cellular_automaton_prg::automaton::rules::parsers::{parse_rules as parse, types};
pub use exitfailure::ExitFailure;
use std::fs;

fn main() -> Result<(), ExitFailure> {
    let path = std::path::Path::new("src/config/rule110.json");

    let data = fs::read_to_string(path).expect("Unable to read file");

    let data: types::OneDimRules = parse(&data)?;

    for rule in data.rules {
        println!("{:?}", rule.neighborhood);
    }

    Ok(())
}
