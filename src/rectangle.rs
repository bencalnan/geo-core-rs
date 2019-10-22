#![allow(dead_code)]
use crate::point::Point;
use crate::point::GeomType;

pub struct Rectangle<T> {
    pub p1: Point<T>,
    pub p2: Point<T>,
}

impl <T>Rectangle<T> {
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    fn translate<T>(&mut self, x: T, y: T) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}


impl <T> GeomType for Rectangle<T> {
    fn describe(&self) -> String {
        String::from("rectangle")
    }
}

