use anyhow::{anyhow, bail, Context, Error, Result};
use regex::Regex;
use std::iter::Peekable;

pub type Number = i64;

#[derive(Debug)]
pub struct Machine {
    pub x_a: Number,
    pub y_a: Number,
    pub x_b: Number,
    pub y_b: Number,
    pub x_p: Number,
    pub y_p: Number,
}

#[derive(Debug)]
pub struct Input {
    pub machines: Vec<Machine>,
}

impl Input {}

pub struct Parser {
    re_button: Regex,
    re_prize: Regex,
}

/*
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400
*/
impl Parser {
    pub fn new() -> Parser {
        Parser {
            re_button: Regex::new(
                r"^Button (?<button>A|B): X(?<x>(?:\+|\-)\d+), Y(?<y>(?:\+|\-)\d+)$",
            )
            .unwrap(),
            re_prize: Regex::new(r"^Prize: X=(?<x>\d+), Y=(?<y>\d+)$").unwrap(),
        }
    }

    pub fn parse(&self, s: &str) -> Result<Input, Error> {
        let mut lines = s.lines().peekable();
        let mut machines: Vec<Machine> = Vec::new();
        while let Some(machine) = self.parse_machine(&mut lines)? {
            machines.push(machine);
        }
        Ok(Input { machines })
    }

    fn parse_machine<'a, 'b, I>(
        &'a self,
        lines: &'b mut Peekable<I>,
    ) -> Result<Option<Machine>, Error>
    where
        I: Iterator<Item = &'a str>,
    {
        // Skip empty lines.
        while lines.next_if(|l| l.trim().is_empty()).is_some() {}
        if lines.peek().is_none() {
            return Ok(None);
        }

        let (name, x_a, y_a) =
            self.parse_button(lines.next().ok_or(anyhow!("Missing Button A line"))?)?;
        if name != "A" {
            bail!("expected button A, actual was button {name}");
        }
        let (name, x_b, y_b) =
            self.parse_button(lines.next().ok_or(anyhow!("Missing Button B line"))?)?;
        if name != "B" {
            bail!("expected button B, actual was button {name}");
        }

        let prize_caps = self
            .re_prize
            .captures(lines.next().ok_or(anyhow!("Missing Prize line"))?)
            .ok_or(anyhow!("invalid Prize"))?;
        let x_p: Number = prize_caps["x"].parse().context("invalid x prize")?;
        let y_p: Number = prize_caps["y"].parse().context("invalid y prize")?;

        Ok(Some(Machine {
            x_a,
            y_a,
            x_b,
            y_b,
            x_p,
            y_p,
        }))
    }

    fn parse_button(&self, l: &str) -> Result<(String, Number, Number), Error> {
        let caps = self
            .re_button
            .captures(l)
            .ok_or(anyhow!("invalid button A"))?;
        let button = caps["button"].to_string();
        let x: Number = caps["x"].parse().context("failed to parse X coefficient")?;
        let y: Number = caps["y"].parse().context("failed to parse Y coefficient")?;
        Ok((button, x, y))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let s = "
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176
";
        let parser = Parser::new();
        let input = parser.parse(s).unwrap();
        println!("{:?}", input);
    }
}
