use anyhow::{bail, Error};

mod part1;
mod part2;
mod input;

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        bail!("Expected <filename>");
    }
    let filename = &args[1];
    let input_data = std::fs::read_to_string(filename)?;
    let input = input::Input::parse(&input_data)?;

    println!("part1={}", part1::eval(&input));
    println!("part2={}", part2::eval(&input));

    Ok(())
}
