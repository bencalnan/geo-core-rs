#![allow(dead_code)]
use crate::geometry::Geometry;
use crate::point::Point;
use crate::rectangle::Rectangle;

pub struct Line {
    pub coords: [Point; 2],
}

impl Line {
    fn new(a: Point, b: Point) -> Line {
        Line { coords: [a, b] }
    }

    fn bbox(&self) -> Rectangle {
        Rectangle {
            p1: Point {
                x: self.coords[0].x.min(self.coords[1].x),
                y: self.coords[0].y.min(self.coords[1].y),
            },
            p2: Point {
                x: self.coords[0].x.max(self.coords[1].x),
                y: self.coords[0].y.max(self.coords[1].y),
            },
        }
    }
    //The Euclidean distance between two points of the plane with Cartesian coordinates
    //Calculates Hypoteneuse between two Points.
    fn length(&self) -> f64 {
        ((self.coords[1].x - self.coords[0].x).powi(2)
            + (self.coords[1].y - self.coords[0].y).powi(2))
        .sqrt()
    }

    fn centroid(&self) -> Point {
        Point {
            x: (self.coords[0].x + self.coords[1].x).abs() / 2.0,
            y: (self.coords[0].y + self.coords[1].y).abs() / 2.0,
        }
    }
}

// Traits
impl Geometry for Line {
    fn describe(&self) -> String {
        String::from("line")
    }
}
