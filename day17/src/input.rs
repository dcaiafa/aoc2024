use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
  pub a: i32,
  pub b: i32,
  pub c: i32,
  pub prog: Vec<i32>,
}

impl Input {
  pub fn parse(s: &str) -> Input {
    let lines: Vec<&str> = s
      .lines()
      .map(|l| l.trim())
      .filter(|l| !l.is_empty())
      .collect();
    assert_eq!(lines.len(), 4);
    Input {
      a: parse_kv(lines[0], "Register A").parse().unwrap(),
      b: parse_kv(lines[1], "Register B").parse().unwrap(),
      c: parse_kv(lines[2], "Register C").parse().unwrap(),
      prog: parse_kv(lines[3], "Program")
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect(),
    }
  }
}

fn parse_kv<'a, 'b>(s: &'a str, expected: &'b str) -> &'a str {
  let (key, value) = s.split_once(":").unwrap();
  assert_eq!(key, expected);
  value.trim()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn parse() {
    let s = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
    let input = Input::parse(s);
    assert_eq!(
      input,
      Input {
        a: 729,
        b: 0,
        c: 0,
        prog: vec![0, 1, 5, 4, 3, 0]
      }
    )
  }
}
