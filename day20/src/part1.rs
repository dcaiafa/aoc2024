use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct V(pub i32, pub i32);

impl V {
  pub fn x(&self) -> i32 {
    self.0
  }
  pub fn y(&self) -> i32 {
    self.1
  }
}

impl std::ops::Add for V {
  type Output = V;

  fn add(self, rhs: Self) -> Self::Output {
    V(self.x() + rhs.x(), self.y() + rhs.y())
  }
}

impl std::ops::Sub for V {
  type Output = V;

  fn sub(self, rhs: Self) -> Self::Output {
    V(self.x() - rhs.x(), self.y() - rhs.y())
  }
}

struct Input {
  width: i32,
  height: i32,
  start: V,
  end: V,
  rows: Vec<Vec<bool>>,
}

impl Input {
  pub fn parse(s: &str) -> Input {
    let mut start: Option<V> = None;
    let mut end: Option<V> = None;
    let mut rows: Vec<Vec<bool>> = Vec::new();
    for l in s.lines() {
      let mut row = if rows.is_empty() {
        Vec::new()
      } else {
        Vec::with_capacity(rows[0].len())
      };
      for (i, c) in l.trim().chars().enumerate() {
        row.push(match c {
          '#' => false,
          '.' => true,
          'S' => {
            assert!(start.is_none());
            start = Some(V(i as i32, rows.len() as i32));
            true
          }
          'E' => {
            assert!(end.is_none());
            end = Some(V(i as i32, rows.len() as i32));
            true
          }
          _ => panic!("invalid map character"),
        });
      }
      rows.push(row);
    }

    Input {
      width: rows[0].len() as i32,
      height: rows.len() as i32,
      start: start.unwrap(),
      end: end.unwrap(),
      rows,
    }
  }

  fn is_within_bounds(&self, pos: V) -> bool {
    pos.x() >= 0
      && pos.x() < self.width
      && pos.y() >= 0
      && pos.y() < self.height
  }

  fn is_track(&self, pos: V) -> bool {
    self.is_within_bounds(pos) && self.rows[pos.y() as usize][pos.x() as usize]
  }

  fn goal_dist(&self, pos: V) -> i32 {
    (pos.x() - self.end.x()).abs() + (pos.y() - self.end.y()).abs()
  }
}

#[derive(Clone)]
struct Path {
  pos: V,
  path: Vec<V>,
  score: i32,
  cheat: i32,
}

struct OrdPath(Path);

impl PartialEq for OrdPath {
  fn eq(&self, other: &Self) -> bool {
    self.0.score == other.0.score
  }
}

impl Eq for OrdPath {}

impl PartialOrd for OrdPath {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for OrdPath {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    other.0.score.cmp(&self.0.score)
  }
}

struct SearchBest<'a> {
  input: &'a Input,
  pqueue: BinaryHeap<OrdPath>,
  visited: HashSet<V>,
}

impl<'a> SearchBest<'a> {
  pub fn search(input: &'a Input) -> Option<i32> {
    let mut srch = SearchBest {
      input,
      pqueue: BinaryHeap::new(),
      visited: HashSet::new(),
    };

    srch.run_best()
  }

  fn run_best(&mut self) -> Option<i32> {
    let path = Path {
      pos: self.input.start,
      path: vec![self.input.start],
      score: self.input.goal_dist(self.input.start),
      cheat: 0,
    };
    self.pqueue.push(OrdPath(path));

    while let Some(OrdPath(path)) = self.pqueue.pop() {
      if path.pos == self.input.end {
        // The -1 is because the result is number of steps, not number of
        // positions.
        return Some((path.path.len() as i32) - 1);
      }

      [V(0, -1), V(0, 1), V(-1, 0), V(1, 0)]
        .into_iter()
        .map(|d| path.pos + d)
        .for_each(|pos| {
          if !self.input.is_track(pos) {
            return;
          }
          let already_visited = !self.visited.insert(pos);
          if already_visited {
            return;
          }

          let mut new_path = path.path.clone();
          new_path.push(pos);
          self.pqueue.push(OrdPath(Path {
            pos,
            score: (new_path.len() as i32) + self.input.goal_dist(pos),
            path: new_path,
            cheat: 0,
          }));
        });
    }

    None
  }
}

struct SearchCheat<'a> {
  input: &'a Input,
  pqueue: BinaryHeap<OrdPath>,
  memo: HashMap<(V, i32), i32>,
  best: i32,
  count: i32,
}

impl<'a> SearchCheat<'a> {
  pub fn search(input: &'a Input, best: i32) -> i32 {
    let mut srch = SearchCheat {
      input,
      pqueue: BinaryHeap::new(),
      memo: HashMap::new(),
      best,
      count: 0,
    };
    srch.run();
    srch.count
  }

  fn run(&mut self) {
    let path = Path {
      pos: self.input.start,
      path: vec![self.input.start],
      score: self.input.goal_dist(self.input.start),
      cheat: 2,
    };
    self.pqueue.push(OrdPath(path));

    while let Some(OrdPath(path)) = self.pqueue.pop() {
      [V(0, -1), V(0, 1), V(-1, 0), V(1, 0)]
        .into_iter()
        .map(|d| path.pos + d)
        .for_each(|pos| {
          if !self.input.is_track(pos) {
            return;
          }
          if let Some(

          let already_visited = !self.visited.insert(pos);
          if already_visited {
            return;
          }

          let mut new_path = path.path.clone();
          new_path.push(pos);
          self.pqueue.push(OrdPath(Path {
            pos,
            score: (new_path.len() as i32) + self.input.goal_dist(pos),
            path: new_path,
            cheat: 0,
          }));
        });
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn search_best() {
    let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";
    let input = Input::parse(input);
    println!("end={:?}", input.end);
    let best = SearchBest::search(&input).unwrap();
    assert_eq!(best, 84);
  }
}
