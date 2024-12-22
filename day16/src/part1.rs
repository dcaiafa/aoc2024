use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
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

struct Path {
    path: Vec<V>,
    pos: V,
    dir: V,
    score: i64,
    search_score: i64,
}

impl Path {
    pub fn apply_step(&self, rotate: i32, maze: &Maze) -> Option<Path> {
        let mut dir = self.dir;
        let mut score = self.score;
        match rotate {
            0 => {
                score += 1;
            }
            1 => {
                dir = dir.rotate90();
                score += 1001;
            }
            2 => {
                dir = dir.rotate90().rotate90();
                score += 2001;
            }
            3 => {
                dir = dir.rotate90().rotate90().rotate90();
                score += 1001;
            }
            _ => panic!("not reached"),
        }

        let pos = self.pos + dir;
        if !maze.is_valid_pos(pos) {
            return None;
        }

        let mut path = self.path.clone();
        path.push(pos);

        let search_score = score + min_dist(pos, maze.goal);

        Some(Path {
            path,
            pos,
            dir,
            score,
            search_score,
        })
    }
}

struct OrdPath(Path);

impl PartialEq for OrdPath {
    fn eq(&self, other: &Self) -> bool {
        self.0.search_score == other.0.search_score
    }
}

impl Eq for OrdPath {}

impl Ord for OrdPath {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.search_score.cmp(&self.0.search_score)
    }
}

impl PartialOrd for OrdPath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Search<'a> {
    maze: &'a Maze,
    pqueue: BinaryHeap<OrdPath>,
    seen: HashSet<(V, V)>,
}

impl<'a> Search<'a> {
    pub fn search(maze: &Maze) -> Option<i64> {
        let mut srch = Search {
            maze,
            pqueue: BinaryHeap::new(),
            seen: HashSet::new(),
        };

        srch.run()
    }

    fn run(&mut self) -> Option<i64> {
        let initial_path = Path {
            path: Vec::new(),
            pos: self.maze.start,
            dir: V(1, 0),
            score: 0,
            search_score: min_dist(self.maze.start, self.maze.goal),
        };

        self.seen.insert((initial_path.pos, initial_path.dir));
        self.pqueue.push(OrdPath(initial_path));

        while !self.pqueue.is_empty() {
            let OrdPath(path) = self.pqueue.pop().unwrap();

            println!("pop {:?} {} {}", path.pos, path.score, path.search_score);

            if path.pos == self.maze.goal {
                return Some(path.score);
            }

            [
                path.apply_step(0, self.maze),
                path.apply_step(1, self.maze),
                path.apply_step(2, self.maze),
                path.apply_step(3, self.maze),
            ]
            .into_iter()
            .flatten()
            .inspect(|p| println!("push {:?} {} {}", p.pos, p.score, p.search_score))
            .for_each(|path| {
                if self.seen.insert((path.pos, path.dir)) {
                    self.pqueue.push(OrdPath(path));
                }
            });
        }

        None
    }
}

fn min_dist(a: V, b: V) -> i64 {
    let d = a - b;
    (d.x().abs() + d.y().abs()) as i64
}
