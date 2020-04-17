use crate::automaton::automaton_builder::AutomatonBuilder;
use crate::automaton::rules::parsers::parse_rules as parse;
use crate::automaton::rules::Rules;
use crate::automaton::Automaton;
use crate::utils::string_to_vector;
use exitfailure::ExitFailure;
use failure::ResultExt;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

pub fn cli() -> Result<Automaton, ExitFailure> {
    let opt = Opt::from_args();

    let data = fs::read_to_string(opt.config)?;

    let schema = parse(&data)?;

    // let dims = schema.dimensions;

    let rules = Rules::from(&schema);

    rules.verify_dimensions(schema.dimensions.len())?;

    let mut ab = AutomatonBuilder::new(schema.dimensions);
    let automaton = ab.set_rules(rules).build()?;

    Ok(automaton)
}

/// This tool allows you to encrypt any number of fields
/// with AES (choosing from a few flavors). It relies on
/// the openssl implementation. Output is in base 64. See
/// https://docs.rs/openssl/0.9.17/openssl/symm/struct.Cipher.html
/// for more information.
#[derive(StructOpt, Debug)]
#[structopt(name = "cellular automaton", version = "0.2.6", author = "")]
struct Opt {
    /// Path to cellular automata config file
    #[structopt(long = "config", parse(from_os_str), raw(required = "true"))]
    config: PathBuf,
    /// Fields to add
    #[structopt(long = "coordinates")]
    fields: Vec<String>,
}
