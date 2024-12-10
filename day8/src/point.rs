use std::ops::{Add, Sub};

#[derive(Debug,Clone,Copy,Hash,PartialEq,Eq)]
pub struct Point(pub i32, pub i32);

impl Point {
    pub fn x(&self) -> i32 { return self.0 }
    pub fn y(&self) -> i32 { return self.1 }
    pub fn in_rect(&self, w: i32, h: i32) -> bool {
        self.x() >= 0 && self.x() < w && self.y() >= 0 && self.y() < h
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point(self.x() + other.x(), self.y() + other.y())
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point(self.x() - other.x(), self.y() - other.y())
    }
}
