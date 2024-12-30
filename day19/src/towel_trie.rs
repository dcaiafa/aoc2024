#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
  W = 0,
  U = 1,
  B = 2,
  R = 3,
  G = 4,
}

impl Color {
  const COUNT: usize = 5;

  pub fn from_char(ch: char) -> Option<Color> {
    match ch {
      'w' => Some(Color::W),
      'u' => Some(Color::U),
      'b' => Some(Color::B),
      'r' => Some(Color::R),
      'g' => Some(Color::G),
      _ => None,
    }
  }

  pub fn from_str(s: &str) -> Option<Vec<Color>> {
    s.chars()
      .map(|c| Color::from_char(c))
      .collect::<Option<Vec<Color>>>()
  }
}

#[derive(Default)]
struct Node {
  is_word: bool,
  next: [Option<u16>; Color::COUNT],
}

pub struct TowelTrie {
  nodes: Vec<Node>,
}

impl TowelTrie {
  pub fn new() -> TowelTrie {
    TowelTrie {
      nodes: vec![Node {
        ..Default::default()
      }],
    }
  }

  pub fn insert(&mut self, colors: &[Color]) -> bool {
    assert!(!colors.is_empty());

    let mut colors = colors;
    let mut node = 0;
    let mut inserted = false;

    while !colors.is_empty() {
      let color = colors[0];
      colors = &colors[1..];

      if let Some(next) = self.nodes[node].next[color as usize] {
        node = next as usize;
      } else {
        let new_node = self.new_node();
        self.nodes[node].next[color as usize] = Some(new_node as u16);
        inserted = true;
        node = new_node;
      }
    }

    if !self.nodes[node].is_word {
      self.nodes[node].is_word = true;
      inserted = true;
    }

    inserted
  }

  pub fn advance(&self, node: usize, c: Color) -> Option<usize> {
    self.nodes[node].next[c as usize].and_then(|n| Some(n as usize))
  }

  pub fn is_word(&self, node: usize) -> bool {
    self.nodes[node].is_word
  }

  fn new_node<'a>(&'a mut self) -> usize {
    let index = self.nodes.len();
    self.nodes.push(Node {
      ..Default::default()
    });
    index
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn towel_trie() {
    let mut trie = TowelTrie::new();
    trie.insert(&[Color::R]);
    trie.insert(&[Color::W, Color::R]);
    trie.insert(&[Color::B]);
    trie.insert(&[Color::G]);
    trie.insert(&[Color::B, Color::W, Color::U]);
    trie.insert(&[Color::R, Color::B]);
    trie.insert(&[Color::G, Color::B]);
    trie.insert(&[Color::B, Color::R]);

    let mut node = trie.advance(0, Color::B).unwrap();
    assert!(trie.is_word(node));
    node = trie.advance(node, Color::W).unwrap();
    assert!(!trie.is_word(node));
    node = trie.advance(node, Color::U).unwrap();
    assert!(trie.is_word(node));
    assert!(trie.advance(node, Color::R).is_none());
  }
}
