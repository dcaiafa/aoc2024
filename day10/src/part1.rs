use std::collections::HashSet;

use crate::input::Input;

#[derive(Debug,Clone,Copy,Hash,PartialEq,Eq)]
struct Point (i32,i32);

pub fn eval(input: &Input) -> i32 {
    let mut n = 0;
    for y in 0..input.height() {
        for x in 0..input.width() {
            if input.get(x, y) == 0 {
                n += trails(&input, x, y, 0, &mut HashSet::new());
            }
        }
    }
    n
}

fn trails(input: &Input, x: i32, y: i32, e: u8, dests: &mut HashSet<Point>) -> i32 {
    if x < 0 || x >= input.width() || y < 0 || y >= input.height() {
        return 0;
    }
    let v = input.get(x, y);
    if v != e {
        return 0;
    }
    if v == 9 {
        if dests.insert(Point(x,y)) {
            return 1;
        } else {
            return 0;
        }
    }

    let e = e + 1;
    trails(input, x+1, y, e, dests) +
        trails(input, x, y+1, e, dests) +
        trails(input, x-1, y, e, dests) +
        trails(input, x, y-1, e, dests)
}
