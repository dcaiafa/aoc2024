use anyhow::{anyhow, Error};
use regex::Regex;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct V(pub i32, pub i32);

impl V {
    pub fn x(&self) -> i32 {
        self.0
    }
    pub fn y(&self) -> i32 {
        self.1
    }
}

#[derive(Debug, Clone)]
pub struct Robot {
    pub pos: V,
    pub vel: V,
}

#[derive(Debug)]
pub struct Input {
    pub robots: Vec<Robot>,
}

impl Input {
    pub fn parse(s: &str) -> Result<Input, Error> {
        let re = Regex::new(r"^p=(?<px>\d+),(?<py>\d+)\s+v=(?<vx>-?\d+),(?<vy>-?\d+)\s*$").unwrap();

        let mut robots = Vec::new();
        for l in s.lines() {
            let caps = re.captures(l).ok_or_else(|| anyhow!("invalid line: {l}"))?;
            let robot = Robot {
                pos: V(caps["px"].parse()?, caps["py"].parse()?),
                vel: V(caps["vx"].parse()?, caps["vy"].parse()?),
            };
            robots.push(robot);
        }

        Ok(Input { robots })
    }
}
