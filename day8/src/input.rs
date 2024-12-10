use std::collections::HashMap;
use crate::point::Point;

use anyhow::{bail, Error};

pub struct Input {
    pub width: i32,
    pub height: i32,
    pub antennae: HashMap<char, Vec<Point>>,
}

impl Input {
    pub fn parse(r: &str) -> Result<Input, Error> {
        let lines = r.lines();
        let mut height: i32 = 0;
        let mut width: Option<i32> = None;
        let mut antennae: HashMap<char, Vec<Point>> = HashMap::new();
        for (y, line) in lines.into_iter().enumerate() {
            height += 1;
            let line = line.trim();
            if let Some(width) = width {
                if width != line.len().try_into()? {
                    bail!("inconsistent width")
                }
            } else {
                width = Some(line.len().try_into()?);
            }

            for (x, c) in line.chars().enumerate() {
                if c == '.' {
                    continue;
                }
                antennae
                    .entry(c)
                    .or_default()
                    .push(Point(x as i32, y as i32));
            }
        }
        if width.is_none() {
            bail!("input is empty");
        }

        Ok(Input { width: width.unwrap(), height, antennae })
    }
}
