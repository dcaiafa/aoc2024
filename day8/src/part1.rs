use std::collections::HashSet;

use crate::comb::comb;
use crate::input::Input;
use crate::point::Point;
use anyhow::Error;

pub fn part1(input: &Input) -> Result<usize, Error> {
    let mut antinodes: HashSet<Point> = HashSet::new();
    for (_antenna, points) in &input.antennae {
        for comb_points in comb(points, 2) {
            let a = *comb_points[0];
            let b = *comb_points[1];
            let d = b - a;
            let sa = a - d;
            let sb = b + d;
            if sa.in_rect(input.width, input.height) {
                antinodes.insert(sa);
            }
            if sb.in_rect(input.width, input.height) {
                antinodes.insert(sb);
            }
        }
    }

    Ok(antinodes.len())
}
