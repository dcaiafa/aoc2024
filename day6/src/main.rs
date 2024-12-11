use std::collections::HashSet;

use anyhow::{bail, Error};

fn main() -> Result<(), Error> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        bail!("Expected <filename>")
    }
    let filename = &args[1];
    let input = std::fs::read_to_string(filename)?;
    let map = Map::parse(&input)?;

    let path = part1(&map);
    println!("part1 = {}", path.len());
    println!("part2 = {}", part2(&map, &path));

    Ok(())
}

fn part1(m: &Map) -> HashSet<V2> {
    let mut guard = m.guard_start;
    let mut guard_dir = V2 { x: 0, y: -1 };
    let mut path: HashSet<V2> = HashSet::new();
    path.insert(guard);
    loop {
        let new_pos = guard.add(&guard_dir);
        if new_pos.y < 0 || new_pos.y >= m.height || new_pos.x < 0 || new_pos.x >= m.width {
            break;
        }
        if m.obstacles.contains(&new_pos) {
            guard_dir = guard_dir.rotate90();
            continue;
        }
        guard = new_pos;
        path.insert(guard);
    }

    return path;
}

fn part2(m: &Map, path: &HashSet<V2>) -> i32 {
    let mut c = 0;

    for v in path {
        if is_loop(m, v) {
            c += 1;
        }
    }
    return c;
}

fn is_loop(m: &Map, obstacle: &V2) -> bool {
    if m.obstacles.contains(&obstacle) {
        return false;
    }

    let mut guard = m.guard_start;
    let mut guard_dir = V2 { x: 0, y: -1 };
    let mut path: HashSet<(V2, V2)> = HashSet::new();
    path.insert((guard, guard_dir));
    loop {
        let new_pos = guard.add(&guard_dir);
        if new_pos.y < 0 || new_pos.y >= m.height || new_pos.x < 0 || new_pos.x >= m.width {
            return false;
        }
        if m.obstacles.contains(&new_pos) || new_pos == *obstacle {
            guard_dir = guard_dir.rotate90();
            continue;
        }
        guard = new_pos;
        if !path.insert((guard, guard_dir)) {
            return true;
        }
    }
}

fn dump(m: &Map, p: &HashSet<V2>) {
    for y in 0..m.height {
        for x in 0..m.width {
            let v = V2 { x, y };
            if m.obstacles.contains(&v) {
                print!("#");
            } else if p.contains(&v) {
                print!("X");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct V2 {
    pub x: i32,
    pub y: i32,
}

impl V2 {
    fn rotate90(&self) -> V2 {
        V2 {
            x: -self.y,
            y: self.x,
        }
    }
    fn add(&self, v: &V2) -> V2 {
        V2 {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }
}

struct Map {
    pub width: i32,
    pub height: i32,
    pub obstacles: HashSet<V2>,
    pub guard_start: V2,
}

impl Map {
    fn parse(input: &str) -> Result<Map, Error> {
        let lines: Vec<String> = input
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(String::from)
            .collect();

        let height: i32 = lines.len().try_into()?;
        if height == 0 {
            bail!("Map has no height");
        }
        let width: i32 = lines[0].len().try_into()?;
        if width == 0 {
            bail!("Map has no width");
        }

        let mut guard_start: Option<V2> = None;
        let mut obstacles: HashSet<V2> = HashSet::new();
        for (y, line) in lines.iter().enumerate() {
            if line.len() != width as usize {
                bail!("Map has inconsistent width");
            }
            for (x, c) in line.chars().into_iter().enumerate() {
                match c {
                    '.' => {}
                    '#' => {
                        obstacles.insert(V2 {
                            x: x as i32,
                            y: y as i32,
                        });
                    }
                    '^' => {
                        if guard_start.is_some() {
                            bail!("Guard specified more than once");
                        }
                        guard_start = Some(V2 {
                            x: x as i32,
                            y: y as i32,
                        });
                    }
                    _ => {
                        bail!("Invalid map character");
                    }
                };
            }
        }

        if guard_start.is_none() {
            bail!("Map has no guard");
        }

        Ok(Map {
            width,
            height,
            obstacles,
            guard_start: guard_start.unwrap(),
        })
    }
}
