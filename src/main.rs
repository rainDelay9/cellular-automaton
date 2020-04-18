use cellular_automaton::automaton::Automaton;
use cellular_automaton::cli::cli;
use exitfailure::ExitFailure;
use std::io::{self, Write};

fn main() -> Result<(), ExitFailure> {
    let mut automaton = cli()?;

    let mut play = true;

    while play {
        println!("");
        println!("{}\n", automaton);
        println!("Select an option:");
        println!("1: advance generation");
        println!("2: advance many generations");
        println!("3: exit");
        println!("");
        print!("selection: ");
        io::stdout().flush().unwrap();

        let mut selection = String::new();
        io::stdin().read_line(&mut selection)?;
        let selection: u32 = selection.trim().parse()?;

        match selection {
            1 => automaton.advance()?,
            2 => advance_many_generations_handler(&mut automaton)?,
            3 => {
                println!("\nGoodbye!\n");
                play = false;
            }
            _ => println!("please select from one of the options"),
        }
    }

    Ok(())
}

fn advance_many_generations_handler(automaton: &mut Automaton) -> Result<(), ExitFailure> {
    loop {
        print!("enter generations to advance: ");
        io::stdout().flush().unwrap();
        let mut generations = String::new();
        io::stdin().read_line(&mut generations)?;
        let generations: u32 = match generations.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        automaton.advance_multi(generations)?;
        break;
    }
    Ok(())
}
