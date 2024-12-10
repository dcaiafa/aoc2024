mod comb;
mod input;
mod part1;
mod part2;
mod point;

use anyhow::{bail, Error};

fn main() -> Result<(), Error> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2{
        bail!("Expected <filename>")
    }
    let filename = &args[1];
    let input_data = std::fs::read_to_string(filename)?;
    let input = input::Input::parse(&input_data)?;
    println!("part1={}", part1::part1(&input)?);
    println!("part2={}", part2::part2(&input)?);

    Ok(())
}
