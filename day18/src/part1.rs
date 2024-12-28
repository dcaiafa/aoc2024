use std::collections::{BinaryHeap, HashSet};

use crate::input::{Input, V};

struct Grid {
  pub width: i32,
  pub height: i32,
  pub walls: HashSet<V>,
}

struct Path {
  pub path: Vec<V>,
  pub pos: V,
}

impl Path {
  pub fn step(&self, dir: V, grid: &Grid) -> Option<Path> {
    let pos = self.pos + dir;
    if pos.x() < 0
      || pos.x() >= grid.width
      || pos.y() < 0
      || pos.y() >= grid.height
    {
      return None;
    }

    if grid.walls.contains(&pos) {
      return None;
    }

    let mut path = self.path.clone();
    path.push(pos);

    Some(Path { path, pos })
  }
}

struct OrdPath(Path, i32);

impl PartialEq for OrdPath {
  fn eq(&self, other: &Self) -> bool {
    self.1 == other.1
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
    other.1.cmp(&self.1)
  }
}

struct Search<'a> {
  grid: &'a Grid,
  pqueue: BinaryHeap<OrdPath>,
  start: V,
  goal: V,
  visited: HashSet<V>,
}

impl<'a> Search<'a> {
  pub fn search(grid: &'a Grid) -> Option<Path> {
    assert!(grid.width > 0 && grid.height > 0);
    let start = V(0, 0);
    let goal = V(grid.width - 1, grid.height - 1);

    let mut s = Search {
      grid,
      pqueue: BinaryHeap::new(),
      start,
      goal,
      visited: HashSet::new(),
    };

    s.run()
  }

  fn run(&mut self) -> Option<Path> {
    let start = Path {
      path: vec![self.start],
      pos: self.start,
    };
    self.push_path(start);

    while let Some(OrdPath(path, _score)) = self.pqueue.pop() {
      //println!("{}", _score);
      //dump_path(&path, self.grid);

      if path.pos == self.goal {
        return Some(path);
      }

      [
        path.step(V(0, -1), self.grid), // up
        path.step(V(0, 1), self.grid),  // down
        path.step(V(1, 0), self.grid),  // right
        path.step(V(-1, 0), self.grid), // left
      ]
      .into_iter()
      .flatten() // Remove Nones
      .for_each(|path| {
        if self.visited.insert(path.pos) {
          self.push_path(path);
        }
      });
    }

    None
  }

  fn push_path(&mut self, path: Path) {
    let score = (path.path.len() as i32) + self.dist(path.pos);
    self.pqueue.push(OrdPath(path, score));
  }

  fn dist(&self, pos: V) -> i32 {
    (self.goal.x() - pos.x()).abs() + (self.goal.y() - pos.y()).abs()
  }
}

#[allow(dead_code)]
fn dump_path(path: &Path, grid: &Grid) {
  let mut grid: Vec<Vec<char>> = (0..grid.height)
    .map(|j| {
      (0..grid.width)
        .map(move |i| match grid.walls.get(&V(i as i32, j as i32)) {
          Some(_) => '#',
          None => '.',
        })
        .collect()
    })
    .collect();

  for pos in &path.path {
    grid[pos.y() as usize][pos.x() as usize] = 'O';
  }

  for row in grid {
    println!("{}", row.iter().collect::<String>());
  }
}

pub fn eval(
  input: &str,
  width: i32,
  height: i32,
  byte_count: i32,
) -> Option<i32> {
  let input = Input::parse(input);

  let walls: HashSet<V> = input
    .coords
    .iter()
    .copied()
    .take(byte_count as usize)
    .collect();

  let grid = Grid {
    width,
    height,
    walls,
  };

  let path = Search::search(&grid);

  // The -1 is because the problem is looking for the number of steps, not the
  // number of positions.
  path.and_then(|p| Some((p.path.len() as i32) - 1))
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn sample() {
    let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";
    assert_eq!(eval(input, 7, 7, 12).unwrap(), 22);
  }
}
