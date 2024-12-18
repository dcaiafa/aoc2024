use crate::input::{Grid, Input, Move, Object, V};

pub fn eval(input: &Input) -> i64 {
    let mut robot = input.robot;
    let mut grid = input.grid.clone();

    //println!("Start:");
    //grid.dump();

    for m in &input.moves {
        let dir = match m {
            Move::Up => V(0, -1),
            Move::Down => V(0, 1),
            Move::Left => V(-1, 0),
            Move::Right => V(1, 0),
        };

        if push(&mut grid, robot, dir) {
            robot = robot + dir;
        }

        //println!("After {:?}:", m);
        //grid.dump();
    }

    score(&grid)
}

fn push(grid: &mut Grid, p: V, d: V) -> bool {
    match grid.get(p.x(), p.y()) {
        None => true,
        Some(Object::Wall) => false,
        Some(Object::Robot) | Some(Object::Box) => {
            if push(grid, p + d, d) {
                grid.mv(p, p + d);
                return true;
            } else {
                return false;
            }
        }
    }
}

fn score(grid: &Grid) -> i64 {
    (0..grid.height)
        .map(|j| (0..grid.width).map(move |i| (i, j)))
        .flatten()
        .filter(|&(i, j)| grid.get(i, j) == Some(Object::Box))
        .map(|(i, j)| (j * 100 + i) as i64)
        .sum()
}
