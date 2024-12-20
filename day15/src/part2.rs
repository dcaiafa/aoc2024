use anyhow::{anyhow, Error};
use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct V(pub i32, pub i32);

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ObjectType {
    Wall,
    Box,
    Robot,
}

impl ObjectType {
    fn width(self) -> i32 {
        match self {
            ObjectType::Robot => 1,
            _ => 2,
        }
    }
}

type ObjectId = usize;

#[derive(Clone)]
struct Grid {
    objects: Vec<ObjectType>,
    coord_to_object: HashMap<V, ObjectId>,
    object_to_coords: HashMap<ObjectId, Vec<V>>,
}

impl Grid {
    pub fn parse<'a, I>(lines: I) -> Result<Grid, Error>
    where
        I: Iterator<Item = &'a str>,
    {
        let data = lines
            .enumerate()
            .map(|(j, l)| {
                l.chars().enumerate().filter_map(move |(i, c)| {
                    let pos = V((i * 2) as i32, j as i32);
                    match c {
                        '#' => Some(Ok((pos, ObjectType::Wall))),
                        'O' => Some(Ok((pos, ObjectType::Box))),
                        '@' => Some(Ok((pos, ObjectType::Robot))),
                        '.' => None,
                        _ => Some(Err(anyhow!("invalid map character {c}"))),
                    }
                })
            })
            .flatten();

        let mut objects: Vec<ObjectType> = Vec::new();
        let mut coord_to_object: HashMap<V, ObjectId> = HashMap::new();
        let mut object_to_coords: HashMap<ObjectId, Vec<V>> = HashMap::new();

        for d in data {
            let (pos, obj_type) = d?;
            let object_id = objects.len() as ObjectId;
            objects.push(obj_type);
            let points = (0..obj_type.width()).map(|i| V(pos.x() + i, pos.y()));
            for point in points {
                coord_to_object.insert(point, object_id);
                object_to_coords
                    .entry(object_id)
                    .and_modify(|v| v.push(point))
                    .or_insert(vec![point]);
            }
        }

        Ok(Grid {
            objects,
            coord_to_object,
            object_to_coords,
        })
    }

    #[allow(dead_code)]
    pub fn dump(&self) {
        let V(width, height) = self
            .coord_to_object
            .keys()
            .fold(V(0, 0), |acc, e| V(acc.x().max(e.x()), acc.y().max(e.y())))
            + V(1, 1);

        for j in 0..height {
            for i in 0..width {
                let obj = self.coord_to_object.get(&V(i, j));
                let c = match obj {
                    None => '.',
                    Some(&id) => match self.objects[id] {
                        ObjectType::Wall => '#',
                        ObjectType::Box => 'O',
                        ObjectType::Robot => '@',
                    },
                };
                print!("{}", c);
            }
            println!("");
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

struct Input {
    robot_id: ObjectId,
    grid: Grid,
    moves: Vec<Move>,
}

impl Input {
    pub fn parse(s: &str) -> Result<Input, Error> {
        let mut lines = s.lines().into_iter().map(|l| l.trim());
        let grid_lines = lines.by_ref().take_while(|l| !l.is_empty());
        let grid = Grid::parse(grid_lines)?;
        let robot_id: ObjectId = grid
            .objects
            .iter()
            .position(|&t| t == ObjectType::Robot)
            .ok_or(anyhow!("no robot"))?;

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
            robot_id,
            grid,
            moves,
        })
    }
}

pub fn eval(input: &str) -> Result<i64, Error> {
    let input = Input::parse(input)?;
    let robot_id = input.robot_id;
    let mut grid = input.grid.clone();
    //grid.dump();

    for m in &input.moves {
        let dir = match m {
            Move::Up => V(0, -1),
            Move::Down => V(0, 1),
            Move::Left => V(-1, 0),
            Move::Right => V(1, 0),
        };
        //println!("Move {:?}", m);
        if can_push(&grid, robot_id, dir) {
            push(&mut grid, robot_id, dir);
        }
        //grid.dump();
    }

    Ok(score(&grid))
}

fn can_push(grid: &Grid, id: ObjectId, dir: V) -> bool {
    //println!("can_push {} {:?} {:?}", id, grid.objects[id], dir);
    match grid.objects[id] {
        ObjectType::Wall => return false,
        ObjectType::Box | ObjectType::Robot => {
            grid.object_to_coords[&id]
                .iter()
                .map(|&p| p + dir)
                .map(|neighbor_p| {
                    grid.coord_to_object
                        .get(&neighbor_p)
                        .and_then(|&id| Some(id))
                })
                .filter(|&nid| nid.is_some_and(|nid| nid != id))
                // Remove Nones.
                .flatten()
                // Compute set of unique object-id.
                .collect::<HashSet<ObjectId>>()
                .into_iter()
                .map(|id| can_push(grid, id, dir))
                .all(|b| b)
        }
    }
}

fn push(grid: &mut Grid, id: ObjectId, dir: V) {
    //println!("push {} {:?} {:?}", id, grid.objects[id], dir);

    assert!(grid.objects[id] != ObjectType::Wall);

    grid.object_to_coords[&id]
        .iter()
        .map(|&p| p + dir)
        .map(|neighbor_p| {
            grid.coord_to_object
                .get(&neighbor_p)
                .and_then(|&id| Some(id))
        })
        .filter(|&nid| nid.is_some_and(|nid| nid != id))
        // Remove Nones.
        .flatten()
        // Compute set of unique object-id.
        .collect::<HashSet<ObjectId>>()
        .into_iter()
        .for_each(|neighbor_id| push(grid, neighbor_id, dir));

    let old_points = grid.object_to_coords.remove(&id).unwrap();
    for p in &old_points {
        grid.coord_to_object.remove(p).unwrap();
    }
    let new_points: Vec<V> = old_points.iter().map(|&p| p + dir).collect();
    for p in &new_points {
        let prev = grid.coord_to_object.insert(*p, id);
        assert!(prev.is_none());
    }
    let prev = grid.object_to_coords.insert(id, new_points);
    assert!(prev.is_none());
}

fn score(grid: &Grid) -> i64 {
    grid.objects
        .iter()
        .enumerate()
        .filter_map(|(id, &typ)| {
            if typ == ObjectType::Box {
                Some(id)
            } else {
                None
            }
        })
        .map(|id| (&grid.object_to_coords[&id]).first().unwrap())
        .map(|&p| (p.y() * 100 + p.x()) as i64)
        .sum()
}
