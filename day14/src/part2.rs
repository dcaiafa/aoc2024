use std::collections::HashMap;

use crate::input::{Input, Robot, V};

const SECS: i32 = 10_000;

struct Grid {
    width: i32,
    height: i32,
    robots: Vec<Robot>,
}

impl Grid {
    fn dump(&self) {
        let mut m: HashMap<V, i32> = HashMap::new();
        for r in &self.robots {
            m.entry(r.pos).and_modify(|c| *c += 1).or_insert(1);
        }
        for j in 0..self.height {
            for i in 0..self.width {
                print!(
                    "{}",
                    match m.get(&V(i, j)) {
                        Some(n) => n.to_string(),
                        None => ".".to_string(),
                    }
                )
            }
            println!("");
        }
    }
}

pub fn eval(input: &Input, width: i32, height: i32) -> i32 {
    let mut g = Grid {
        width,
        height,
        robots: input.robots.clone(),
    };

    for i in 0..SECS {
        for r in &mut g.robots {
            move_robot(width, height, r);
        }
        /*
        if (i + 1 - 62) % 101 == 0 {
            println!("{}:", i + 1);
            g.dump();
        }
        */
        let cn = compute_closest_neighbor(&g);
        println!("{},{}", i+1, cn);
        if cn < 400_000_000 {
            g.dump();
        }
    }
    0
}

fn move_robot(w: i32, h: i32, r: &mut Robot) {
    r.pos = V(
        wrap(r.pos.x() + r.vel.x(), w),
        wrap(r.pos.y() + r.vel.y(), h),
    );
}

fn wrap(v: i32, m: i32) -> i32 {
    match v % m {
        v if v < 0 => v + m,
        v => v,
    }
}

fn compute_closest_neighbor(g: &Grid) -> i64 {
    let mut sum: i64 = 0;
    for a in &g.robots {
        for b in &g.robots {
            let d = (a.pos.x() - b.pos.x()).pow(2) + (a.pos.y() - b.pos.y()).pow(2);
            sum += d as i64;
        }
    }
    sum
}
