use std::collections::HashSet;
use std::{cell::RefCell, rc::Rc};

use anyhow::{bail, Error};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point(i32, i32);

impl Point {
    fn x(&self) -> i32 {
        self.0
    }
    fn y(&self) -> i32 {
        self.1
    }
}

struct Area {
    crop_type: char,
    perim: i32,
    points: HashSet<Point>,
}

impl Area {
    fn new(crop_type: char) -> Rc<RefCell<Area>> {
        Rc::new(RefCell::new(Area {
            crop_type,
            perim: 0,
            points: HashSet::new(),
        }))
    }
}

#[derive(Debug, Default, Clone)]
struct Fences {
    t: bool,
    b: bool,
    l: bool,
    r: bool,
}

struct Plot {
    crop_type: char,
    area: Option<Rc<RefCell<Area>>>,
    fences: Fences,
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
                        crop_type: c,
                        area: None,
                        fences: Default::default(),
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

    fn get_mut_plot(&mut self, x: i32, y: i32) -> &mut Plot {
        &mut self.rows[y as usize][x as usize]
    }

    fn is_within_bounds(&self, p: &Point) -> bool {
        p.x() >= 0 && p.x() < self.width && p.y() >= 0 && p.y() < self.height
    }

    fn fill(&mut self, p: Point, area: Rc<RefCell<Area>>) {
        if !self.is_within_bounds(&p) {
            return;
        }
        {
            let plot_type = self.get_plot(p.x(), p.y()).crop_type;
            let mut area = area.borrow_mut();
            if plot_type != area.crop_type {
                return;
            }
            if !area.points.insert(p) {
                return;
            }
        }
        self.get_mut_plot(p.x(), p.y()).area = Some(area.clone());

        self.fill(Point(p.x() - 1, p.y()), area.clone());
        self.fill(Point(p.x() + 1, p.y()), area.clone());
        self.fill(Point(p.x(), p.y() - 1), area.clone());
        self.fill(Point(p.x(), p.y() + 1), area.clone());
    }

    fn get_area_ptr(&self, p: Point) -> Option<*const Area> {
        if !self.is_within_bounds(&p) {
            return None;
        }
        Some(self.get_plot(p.x(), p.y()).area.clone().unwrap().as_ptr() as *const Area)
    }

    fn share_same_area(&self, a: Point, b: Point) -> bool {
        match (self.get_area_ptr(a), self.get_area_ptr(b)) {
            (Some(pa), Some(pb)) if pa == pb => true,
            _ => false,
        }
    }

    fn get_fences(&self, n: char, x:i32, y: i32) -> Fences {
        if !self.is_within_bounds(&Point(x,y)) {
            return Default::default();
        }

        let plot = self.get_plot(x, y);
        if plot.crop_type != n {
            return Default::default();
        }

        return plot.fences.clone();
    }
}

pub fn eval(input: &str) -> Result<i64, Error> {
    let mut grid = Grid::parse(input)?;

    for j in 0..grid.height {
        for i in 0..grid.width {
            let plot = grid.get_plot(i, j);
            if plot.area.is_none() {
                grid.fill(Point(i, j), Area::new(plot.crop_type));
            }
        }
    }
    for j in 0..grid.height {
        for i in 0..grid.width {
            let pp = Point(i, j);
            let fences = Fences {
                l: !grid.share_same_area(pp, Point(i - 1, j)),
                r: !grid.share_same_area(pp, Point(i + 1, j)),
                t: !grid.share_same_area(pp, Point(i, j - 1)),
                b: !grid.share_same_area(pp, Point(i, j + 1)),
            };
            let crop_type = grid.get_plot(i, j).crop_type;
            let mut sides = 0;
            if fences.l && !grid.get_fences(crop_type, i, j-1).l {
                sides += 1;
            }
            if fences.r && !grid.get_fences(crop_type, i, j-1).r {
                sides += 1;
            }
            if fences.t && !grid.get_fences(crop_type, i-1, j).t {
                sides += 1;
            }
            if fences.b && !grid.get_fences(crop_type, i-1, j).b {
                sides +=1;
            }
            let plot = grid.get_mut_plot(i, j);
            plot.fences = fences;
            let area = plot.area.clone().unwrap();
            let mut area = area.borrow_mut();
            area.perim += sides;
        }
    }

    let mut total_price: i64 = 0;
    let mut areas_visited: HashSet<*const Area> = HashSet::new();
    for j in 0..grid.height {
        for i in 0..grid.width {
            let area = grid.get_plot(i, j).area.clone().unwrap();
            if areas_visited.insert(area.as_ptr()) {
                let area = area.borrow();
                let area_price = (area.points.len() as i32) * area.perim;
                total_price += area_price as i64;
            }
        }
    }

    Ok(total_price)
}
