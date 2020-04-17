use cellular_automaton_prg::cli::cli;
use exitfailure::ExitFailure;

fn main() -> Result<(), ExitFailure> {
    cli()?;

    let play = true;

    while play {}

    Ok(())
}
