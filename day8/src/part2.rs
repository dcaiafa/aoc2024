use std::collections::HashSet;

use crate::comb::comb;
use crate::input::Input;
use crate::point::Point;
use anyhow::Error;

pub fn part2(input: &Input) -> Result<usize, Error> {
    let mut antinodes: HashSet<Point> = HashSet::new();
    for (_antenna, points) in &input.antennae {
        for comb_points in comb(points, 2) {
            let mut a = *comb_points[0];
            let mut b = *comb_points[1];
            let d = b - a;
            while a.in_rect(input.width, input.height) {
                antinodes.insert(a);
                a = a - d;
            }
            while b.in_rect(input.width, input.height) {
                antinodes.insert(b);
                b = b + d;
            }
        }
    }

    Ok(antinodes.len())
}
