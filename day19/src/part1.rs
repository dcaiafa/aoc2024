use crate::towel_trie::{Color, TowelTrie};

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

pub fn eval(s: &str) -> usize {
  let input = Input::parse(s);

  let mut trie = TowelTrie::new();
  for towel in &input.towels {
    let colors = Color::from_str(towel).unwrap();
    trie.insert(&colors);
  }

  input
    .designs
    .iter()
    .filter(|&d| is_design_possible(d, &trie))
    .count()
}

fn is_design_possible(design: &str, towels: &TowelTrie) -> bool {
  let design_colors = Color::from_str(design).unwrap();

  let mut node = 0;
  for i in 0..design_colors.len() {
    match towels.advance(node, design_colors[i]) {
      Some(next) => {
        node = next;
        if towels.is_word(node) && is_design_possible(&design[i + 1..], towels)
        {
          return true;
        }
      }
      None => return false,
    }
  }

  true
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

    assert_eq!(eval(input), 6);
  }
}
