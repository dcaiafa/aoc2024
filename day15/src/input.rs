use std::ops::{Add, Sub};

use anyhow::{anyhow, bail, Error};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct V(pub i32, pub i32);

impl V {
    pub fn x(&self) -> i32 {
        self.0
    }
    pub fn y(&self) -> i32 {
        self.1
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Object {
    Wall,
    Box,
    Robot,
}

#[derive(Clone)]
pub struct Grid {
    pub width: i32,
    pub height: i32,
    pub objects: Vec<Vec<Option<Object>>>,
}

impl Grid {
    fn parse<'a, I>(lines: I) -> Result<Grid, Error>
    where
        I: Iterator<Item = &'a str>,
    {
        let objects: Vec<Vec<Option<Object>>> = lines
            .map(|l| {
                l.chars()
                    .map(move |c| match c {
                        '#' => Ok(Some(Object::Wall)),
                        'O' => Ok(Some(Object::Box)),
                        '@' => Ok(Some(Object::Robot)),
                        '.' => Ok(None),
                        _ => Err(anyhow!("invalid map character {c}")),
                    })
                    .collect::<Result<_,_>>()
            })
            .collect::<Result<_,_>>()?;

        let height = objects.len();
        if height == 0 {
            bail!("grid height cannot be zero");
        }
        let width = objects[0].len();
        if width == 0 {
            bail!("grid width cannot be zero");
        }
        if !objects.iter().all(|r|r.len() == width) {
            bail!("grid width is inconsistent");
        }

        Ok(Grid{
            width: width.try_into()?,
            height: height.try_into()?,
            objects
        })
    }

    #[allow(dead_code)]
    pub fn dump(&self) {
        for r in &self.objects {
            for o in r {
                let c = match o {
                    &Some(Object::Wall) => '#',
                    &Some(Object::Box) => 'O',
                    &Some(Object::Robot) => '@',
                    &None => '.',
                };
                print!("{}", c);
            }
            println!("");
        }
    }

    pub fn get(&self, x: i32, y: i32) -> Option<Object> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return None;
        }
        self.objects[y as usize][x as usize]
    }

    pub fn mv(&mut self, from: V, to: V) {
        let from_obj = self.get(from.x(), from.y());
        let to_obj = self.get(to.x(), to.y());
        assert!(from_obj.is_some() && to_obj.is_none());

        self.objects[from.y() as usize][from.x() as usize] = None;
        self.objects[to.y() as usize][to.x() as usize] = from_obj;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

pub struct Input {
    pub robot: V,
    pub grid: Grid,
    pub moves: Vec<Move>,
}

impl Input {
    pub fn parse(s: &str) -> Result<Input, Error> {
        let mut lines = s.lines().into_iter().map(|l| l.trim());
        let grid_lines = lines.by_ref().take_while(|l| !l.is_empty());
        let grid = Grid::parse(grid_lines)?;

        let mut robot: Option<V> = None;
        for j in 0..grid.height {
            for i in 0..grid.width {
                if let Some(Object::Robot) = grid.get(i, j) {
                    if robot.is_some() {
                        bail!("multiple robots");
                    }
                    robot = Some(V(i,j));
                }
            }
        }
        if robot.is_none() {
            bail!("no robot");
        }

        let moves: Vec<Move> = lines
            .map(|l| {
                l.chars().map(|c| match c {
                    '^' => Ok(Move::Up),
                    'v' => Ok(Move::Down),
                    '<' => Ok(Move::Left),
                    '>' => Ok(Move::Right),
                    v => Err(anyhow!("Invalid move {v}")),
                })
            })
            .flatten()
            .collect::<Result<_, _>>()?;

        Ok(Input {
            robot: robot.unwrap(),
            grid,
            moves,
        })
    }
}
