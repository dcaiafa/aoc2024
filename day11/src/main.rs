use anyhow::{bail, Error};

mod part1;
mod part2;

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        bail!("Expected <filename>");
    }
    let filename = &args[1];
    let input = std::fs::read_to_string(filename)?;
    println!("part1={}", part1::eval(&input)?);
    println!("part2={}", part2::eval(&input)?);

    Ok(())
}