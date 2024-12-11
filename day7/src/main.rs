use anyhow::anyhow;
use std::io::{BufRead, Read};

use anyhow::{bail, Error};

fn main() -> Result<(), Error> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        bail!("Expected <filename>");
    }
    let filename = &args[1];
    let file = std::fs::File::open(filename)?;
    let mut file_reader = std::io::BufReader::new(file);
    let input = parse_input(&mut file_reader)?;
    println!("part1={}", part1(&input));
    println!("part2={}", part2(&input));

    Ok(())
}

fn part1(ops: &Vec<Op>) -> i64 {
    let mut sum: i64 = 0;
    for op in ops {
        let n = 2 << op.operands.len()-1;
        for i in 0..n {
            let res = eval1(op, i);
            if res == op.result {
                sum += res;
                break;
            }
        }
    }
    return sum;
}

fn eval1(op: &Op, operators: i32) -> i64 {
    let mut operators = operators;
    let mut res: i64 = op.operands[0];
    for v in &op.operands[1..] {
        res = match operators % 2 {
            0 => res + v,
            _ => res * v,
        };
        operators = operators / 2;
    }
    return res;
}

fn part2(ops: &Vec<Op>) -> i64 {
    let mut sum: i64 = 0;
    for op in ops {
        let n = 3_i32.pow((op.operands.len()-1) as u32);
        for i in 0..n {
            let res = eval2(op, i);
            if res == op.result {
                sum += res;
                break;
            }
        }
    }
    return sum;
}

fn eval2(op: &Op, operators: i32) -> i64 {
    let mut operators = operators;
    let mut res: i64 = op.operands[0];
    for v in &op.operands[1..] {
        res = match operators % 3 {
            0 => res + v,
            1 => res * v,
            _ => (res *l10(*v)) + v,
        };
        operators = operators / 3;
    }
    return res;
}

fn l10(n: i64) -> i64 {
    let mut v = 1; let mut r = n;
    while r > 0 { r /= 10; v *= 10; }
    v
}

struct Op {
    result: i64,
    operands: Vec<i64>,
}

fn parse_input<R: Read + BufRead>(r: &mut R) -> Result<Vec<Op>, Error>
{
    let mut ops = Vec::new();
    let mut line = String::new();
    while {
        line.clear();
        r.read_line(&mut line)? > 0
    } {
        let (result_str, operands_str) = line
            .trim()
            .split_once(":")
            .ok_or(anyhow!("Invalid entry"))?;
        let result: i64 = result_str.parse()?;
        let operands: Vec<i64> = operands_str
            .trim()
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        ops.push(Op{result, operands});
    }

    Ok(ops)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        assert_eq!(eval1(&Op{result: 0, operands: vec![11,6,16,20]}, 0b010), 292);
    }
}
