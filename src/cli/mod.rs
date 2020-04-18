use crate::automaton::automaton_builder::AutomatonBuilder;
use crate::automaton::rules::parsers::{parse_coordinates, parse_rules};
use crate::automaton::rules::Rules;
use crate::automaton::Automaton;
use exitfailure::ExitFailure;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

pub fn cli() -> Result<Automaton, ExitFailure> {
    let opt = Opt::from_args();

    let data = fs::read_to_string(opt.path_to_config)?;

    let schema = parse_rules(&data)?;

    let rules = Rules::from(&schema);

    rules.verify_dimensions(schema.dimensions.len())?;

    let mut ab = AutomatonBuilder::new(schema.dimensions);

    let coordinates_data = fs::read_to_string(opt.path_to_coordinates)?;
    let coordinates_schema = parse_coordinates(&coordinates_data)?;

    for coordinate in coordinates_schema.coordinates {
        ab.set_point(&coordinate[..])?;
    }
    let automaton = ab.set_rules(rules).build()?;

    Ok(automaton)
}

/// This tool allows you to simulate a toroidal cellular automaton with
/// any number of dimensions.
/// To read more about a cellular automata go to:
/// https://mathworld.wolfram.com/CellularAutomaton.html
#[derive(StructOpt, Debug)]
#[structopt(
    name = "cellular-automaton",
    version = "0.1.7",
    author = "Matan Orland (matan.orland@gmail.com)"
)]
struct Opt {
    /// Path to cellular automata config file
    /// (see config/rule110.json as an example)
    #[structopt(long = "config", parse(from_os_str), raw(required = "true"))]
    path_to_config: PathBuf,
    /// Path to cpprdonates config file
    /// coordinates should have the form:
    ///     [x_1,y_1,...]
    ///     [x_2,y_2,...]
    /// in a JSON file (see config/rule110_coordinates.json as an example)
    #[structopt(long = "coordinates", parse(from_os_str), raw(required = "true"))]
    path_to_coordinates: PathBuf,
}
