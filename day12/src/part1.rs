use std::{cell::RefCell, collections::HashSet, rc::Rc};

use anyhow::{bail, Error};

struct Point(i32, i32);

struct Area {
    area: i32,
    perim: i32,
    points: HashSet<Point>,
}

impl Area {
    fn new() -> Rc<RefCell<Area>> {
        Rc::new(RefCell::new(Area {
            area: 0,
            perim: 0,
            points: HashSet::new(),
        }))
    }
}

struct Plot {
    name: char,
    area: Option<Rc<RefCell<Area>>>,
}

struct Grid {
    width: i32,
    height: i32,
    rows: Vec<Vec<Plot>>,
}

impl Grid {
    fn parse(input: &str) -> Result<Grid, Error> {
        let rows: Vec<Vec<Plot>> = input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| Plot {
                        name: c,
                        area: None,
                    })
                    .collect()
            })
            .collect();
        let height = rows.len();
        if height == 0 {
            bail!("grid height is zero");
        }
        let width = rows[0].len();
        if width == 0 {
            bail!("grid width is zero");
        }
        if !rows.iter().all(|r| r.len() == width) {
            bail!("inconsistent width")
        }
        Ok(Grid {
            width: width.try_into()?,
            height: height.try_into()?,
            rows,
        })
    }

    fn get_plot(&self, x: i32, y: i32) -> &Plot {
        &self.rows[y as usize][x as usize]
    }

    fn fill(&mut self, x: i32, y: i32, area: Rc<RefCell<Area>>) {
        if x < 0 || x >= self.width || y < 


        let name = self.get_plot(x, y);
    }
}

pub fn eval(input: &str) -> Result<i32, Error> {
    let mut grid = Grid::parse(input)?;

    for j in 0..grid.height {
        for i in 0..grid.width {
            if grid.get_plot(i, j).area.is_none() {
                grid.fill(i, j, Area::new());
            }
        }
    }

    Ok(())
}
