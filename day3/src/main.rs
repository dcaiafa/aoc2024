use std::{cmp, fs, str};

use anyhow::{bail, Error};

fn main() -> Result<(), Error> {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 2 {
    bail!("Expected input filename");
  }

  let input: &str = &args[1];
  let data = fs::read(input)?;
  println!("eval1: {}", eval(&data));
  println!("eval2: {}", eval2(&data));

  Ok(())
}

fn eval(d: &[u8]) -> i64 {
  let mut sum: i64 = 0;
  let mut d = d;
  while d.len() > 0 {
    if let Some((mul, new_d)) = eval_mul(d) {
      sum += mul;
      d = new_d;
    } else {
      d = &d[1..];
    }
  }
  sum
}

fn eval2(d: &[u8]) -> i64 {
  let mut enabled = true;
  let mut sum: i64 = 0;
  let mut d = d;
  while d.len() > 0 {
    if let Some((mul, new_d)) = eval_mul(d) {
      if enabled {
        sum += mul;
      }
      d = new_d;
    } else if let Some(new_d) = parse_str(d, "do()") {
      enabled = true;
      d = new_d;
    } else if let Some(new_d) = parse_str(d, "don't()") {
      enabled = false;
      d = new_d;
    } else {
      d = &d[1..];
    }
  }
  sum
}

fn eval_mul(d: &[u8]) -> Option<(i64, &[u8])> {
  let d = parse_str(d, "mul(")?;
  let (num1, d) = parse_num(d)?;
  let d = parse_str(d, ",")?;
  let (num2, d) = parse_num(d)?;
  let d = parse_str(d, ")")?;
  Some((num1 * num2, d))
}

fn parse_str<'a>(d: &'a [u8], s: &str) -> Option<&'a [u8]> {
  if d.len() < s.len() {
    return None;
  }
  let sb = s.as_bytes();
  if sb.cmp(&d[0..s.len()]) != cmp::Ordering::Equal {
    return None;
  }
  Some(&d[sb.len()..])
}

fn parse_num<'a>(d: &'a [u8]) -> Option<(i64, &'a [u8])> {
  let mut n = 0;
  while n < d.len() {
    let is_num = d[n] >= b'0' && d[n] <= b'9';
    if !is_num {
      break;
    }
    n += 1;
  }
  if n == 0 {
    return None;
  }
  let num: i64 = str::from_utf8(&d[..n]).ok()?.parse().ok()?;
  Some((num, &d[n..]))
}
