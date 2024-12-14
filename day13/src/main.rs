use anyhow::{bail, Error};

mod input;
mod part1;
mod part2;

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        bail!("expected <filename>");
    }
    let filename = &args[1];
    let filedata = std::fs::read_to_string(filename)?;
    let input = input::Parser::new().parse(&filedata)?;
    println!("part1={}", part1::eval(&input));
    println!("part2={}", part2::eval(&input));
    Ok(())
}
