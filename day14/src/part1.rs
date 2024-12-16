use std::collections::HashMap;

use crate::input::{Input, Robot, V};

const SECS: i32 = 100;

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

    for _ in 0..SECS {
        for r in &mut g.robots {
            move_robot(width, height, r);
        }
    }

    g.dump();

    calc_quads(&g).into_iter().reduce(|acc, e| acc * e).unwrap()
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

fn calc_quads(g: &Grid) -> [i32; 4] {
    let mx = g.width / 2;
    let my = g.height / 2;
    let mut quads = [0; 4];
    for r in &g.robots {
        if r.pos.x() == mx || r.pos.y() == my {
            continue;
        }
        match (r.pos.x() / (mx + 1), r.pos.y() / (my + 1)) {
            (0, 0) => quads[0] += 1,
            (1, 0) => quads[1] += 1,
            (0, 1) => quads[2] += 1,
            _ => quads[3] += 1,
        }
    }
    quads
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample_input() {
        let s = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        let input = Input::parse(s).unwrap();
        assert_eq!(eval(&input, 11, 7), 12);
    }
}
