#![allow(dead_code)]
use crate::point::Point;
use crate::geometry::Geometry;

pub struct Rectangle {
    pub min_point: Point,
    pub max_point: Point,
}

impl Rectangle {
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.min_point;
        let Point { x: x2, y: y2 } = self.max_point;
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.min_point;
        let Point { x: x2, y: y2 } = self.max_point;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    fn translate(&mut self, x: f64, y: f64) {
        self.min_point.x += x;
        self.min_point.x += x;

        self.max_point.y += y;
        self.max_point.y += y;
    }
}
// Traits
impl Geometry for Rectangle {
    fn describe(&self) -> String {
        String::from("rectangle")
    }
}
