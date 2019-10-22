#![allow(dead_code)]
use crate::point::GeomType;
use crate::point::Point;
use crate::rectangle::Rectangle;

pub struct Line<T> {
    pub start: Point<T>,
    pub end: Point<T>,
}

impl <T>Line<T> {
    fn new<T>(a: Point<T>, b: Point<T>) -> Line<T> {
        Line { start: a, end: b }
    }

    fn bbox(&self) -> Rectangle<T> {
        Rectangle {
            p1: Point {
                x: self.start.x.min(self.end.x),
                y: self.start.y.min(self.end.y),
            },
            p2: Point {
                x: self.start.x.max(self.end.x),
                y: self.start.y.max(self.end.y),
            },
        }
    }
    //The Euclidean distance between two points of the plane with Cartesian coordinates
    //Calculates Hypoteneuse between two Points.
    fn length(&self) -> f64 {
        ((self.end.x - self.start.x).powi(2)
            + (self.end.y - self.start.y).powi(2))
        .sqrt()
    }

    fn centroid(&self) -> Point<T> {
        Point {
            x: (self.start.x + self.end.x).abs() / 2.0,
            y: (self.start.y + self.end.y).abs() / 2.0,
        }
    }
}

// Traits
impl<T> GeomType for Line<T> {
    fn describe(&self) -> String {
        String::from("line")
    }
}
