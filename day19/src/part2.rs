use crate::towel_trie::{Color, TowelTrie};
use std::collections::HashMap;

pub struct Input {
  pub towels: Vec<String>,
  pub designs: Vec<String>,
}

impl Input {
  pub fn parse(s: &str) -> Input {
    let mut lines = s.lines();

    let towels_line = lines.next().unwrap();
    let towels = towels_line
      .split(",")
      .map(str::trim)
      .map(String::from)
      .collect();

    // Skip empty line.
    lines.next().unwrap();

    let designs = lines
      .map(str::trim)
      .filter(|l| !l.is_empty())
      .map(String::from)
      .collect::<Vec<String>>();

    Input { towels, designs }
  }
}

pub fn eval(s: &str) -> i64 {
  let input = Input::parse(s);

  let mut trie = TowelTrie::new();
  for towel in &input.towels {
    let colors = Color::from_str(towel).unwrap();
    trie.insert(&colors);
  }

  let mut memo: HashMap<String, i64> = HashMap::new();

  input
    .designs
    .iter()
    .map(|d| design_combinations(d, &trie, &mut memo))
    .sum()
}

fn design_combinations(
  design: &str,
  towels: &TowelTrie,
  memo: &mut HashMap<String, i64>,
) -> i64 {
  if design.len() == 0 {
    return 1;
  }

  if let Some(&combs) = memo.get(design) {
    return combs;
  }

  let design_colors = Color::from_str(design).unwrap();

  let mut combs = 0;
  let mut node = 0;
  for i in 0..design_colors.len() {
    match towels.advance(node, design_colors[i]) {
      Some(next) => {
        node = next;
        if towels.is_word(node) {
          combs += design_combinations(&design[i + 1..], towels, memo);
        }
      }
      None => break,
    }
  }

  memo.insert(design.into(), combs);
  combs
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn sample() {
    let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

    assert_eq!(eval(input), 16);
  }
}
