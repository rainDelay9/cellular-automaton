use crate::automaton::automaton_builder::AutomatonBuilder;
use crate::automaton::rules::parsers::parse_file_to_schema;
use crate::automaton::rules::parsers::schemas::{CoordinatesSchema, DimensionsSchema, RulesSchema};
use crate::automaton::rules::Rules;
use crate::automaton::Automaton;
use exitfailure::ExitFailure;
use std::path::PathBuf;
use structopt::StructOpt;

pub fn cli() -> Result<Automaton, ExitFailure> {
    let opt = Opt::from_args();

    let rules_schema = parse_file_to_schema::<RulesSchema>(&opt.path_to_rules)?;

    let rules = Rules::from(&rules_schema);

    let dimensions_schema = parse_file_to_schema::<DimensionsSchema>(&opt.path_to_dimensions)?;

    rules.verify_dimensions(dimensions_schema.dimensions.len())?;

    let mut ab = AutomatonBuilder::new(dimensions_schema.dimensions);

    let coordinates_schema = parse_file_to_schema::<CoordinatesSchema>(&opt.path_to_coordinates)?;

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
    #[structopt(
        long = "rules",
        short = "r",
        parse(from_os_str),
        raw(required = "true")
    )]
    path_to_rules: PathBuf,

    /// Path to cellular automata config file
    /// (see config/rule110.json as an example)
    #[structopt(
        long = "dimensions",
        short = "d",
        parse(from_os_str),
        raw(required = "true")
    )]
    path_to_dimensions: PathBuf,

    /// Path to cpprdonates config file
    /// coordinates should have the form:
    ///     [x_1,y_1,...]
    ///     [x_2,y_2,...]
    /// in a JSON file (see config/rule110_coordinates.json as an example)
    #[structopt(
        long = "coordinates",
        short = "c",
        parse(from_os_str),
        raw(required = "true")
    )]
    path_to_coordinates: PathBuf,
}
