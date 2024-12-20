use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct V(pub i32, pub i32);

impl V {
    pub fn x(&self) -> i32 {
        self.0
    }
    pub fn y(&self) -> i32 {
        self.1
    }
    pub fn rotate90(&self) -> V {
        V(-self.y(), self.x())
    }
}

impl Add for V {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        V(self.x() + rhs.x(), self.y() + rhs.y())
    }
}

impl Sub for V {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        V(self.x() - rhs.x(), self.y() - rhs.y())
    }
}

struct Maze {
    start: V,
    goal: V,
    width: i32,
    height: i32,
    rows: Vec<Vec<bool>>,
}

impl Maze {
    pub fn parse(s: &str) -> Maze {
        let mut goal: Option<V> = None;
        let mut start: Option<V> = None;
        let mut rows: Vec<Vec<bool>> = Vec::new();

        for (j, line) in s.lines().enumerate() {
            let mut row: Vec<bool> = if j == 0 {
                Vec::new()
            } else {
                Vec::with_capacity(rows[0].len())
            };
            for (i, c) in line.trim().chars().enumerate() {
                match c {
                    '#' => {
                        row.push(false);
                    }
                    '.' => {
                        row.push(true);
                    }
                    'S' => {
                        assert!(start.is_none());
                        start = Some(V(i as i32, j as i32));
                        row.push(true);
                    }
                    'E' => {
                        assert!(goal.is_none());
                        goal = Some(V(i as i32, j as i32));
                        row.push(true);
                    }
                    _ => panic!("invalid maze character"),
                }
            }
            rows.push(row);
        }

        let height = rows.len();
        assert!(height > 0);
        let width = rows[0].len();
        assert!(width > 0);
        assert!(rows.iter().all(|r| r.len() == width));

        Maze {
            start: start.unwrap(),
            goal: goal.unwrap(),
            width: width.try_into().unwrap(),
            height: height.try_into().unwrap(),
            rows,
        }
    }

    pub fn is_valid_pos(&self, pos: V) -> bool {
        pos.x() >= 0
            && pos.x() < self.width
            && pos.y() >= 0
            && pos.y() < self.height
            && self.rows[pos.y() as usize][pos.x() as usize]
    }
}

pub fn eval(s: &str) -> i64 {
    let maze = Maze::parse(s);
    Search::search(&maze).unwrap()
}

struct Search<'a> {
    maze: &'a Maze,
    visited: HashMap<(V, V), i64>,
    max_score: i64,
}

impl<'a> Search<'a> {
    pub fn search(maze: &Maze) -> Option<i64> {
        let mut ctx = Search {
            maze,
            visited: HashMap::new(),
            max_score: i64::MAX,
        };
        ctx.search_step(maze.start, V(1, 0), 0)
    }

    fn search_step(&mut self, pos: V, dir: V, score: i64) -> Option<i64> {
        if !self.maze.is_valid_pos(pos) {
            return None;
        }

        if let Some(&visited_score) = self.visited.get(&(pos, dir)) {
            if score > visited_score {
                return None;
            }
        }
        self.visited.insert((pos, dir), score);

        if score >= self.max_score {
            return Some(self.max_score);
        }

        //println!("step: {:?} {:?} {}", pos, dir, score);
        //self.dump();

        if pos == self.maze.goal {
            println!("Goal! {}", score);
            //self.dump();
            self.max_score = score;
            self.visited.remove(&(pos, dir));
            return Some(score);
        }

        let res = [
            self.search_step(pos + dir, dir, score + 1),
            self.search_step(pos, dir.rotate90(), score + 1000),
            self.search_step(pos, dir.rotate90().rotate90(), score + 2000),
            self.search_step(pos, dir.rotate90().rotate90().rotate90(), score + 1000),
        ]
        .into_iter()
        .flatten()
        .min();

        self.visited.remove(&(pos, dir));

        res
    }

    fn dump(&self) {
        let mut grid: Vec<Vec<char>> = self
            .maze
            .rows
            .iter()
            .map(|r| {
                r.iter()
                    .map(|b| match b {
                        true => '.',
                        false => '#',
                    })
                    .collect()
            })
            .collect();
        for (pos, dir) in self.visited.keys() {
            grid[pos.y() as usize][pos.x() as usize] = match dir {
                V(1, 0) => '>',
                V(-1, 0) => '<',
                V(0, 1) => 'v',
                V(0, -1) => '^',
                _ => panic!("unreachable"),
            };
        }
        grid[self.maze.start.y() as usize][self.maze.start.x() as usize] = 'S';
        grid[self.maze.goal.y() as usize][self.maze.goal.x() as usize] = 'E';

        for r in grid {
            for c in r {
                print!("{}", c);
            }
            println!("");
        }
    }
}
