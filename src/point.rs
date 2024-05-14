#![allow(dead_code)]
use crate::latlng::LatLng;
use crate::geometry::Geometry;

#[derive(Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    fn new_lat_lng(lat_lng: LatLng) -> Point {
        Point {
            x: lat_lng.lng,
            y: lat_lng.lat,
        }
    }

    fn add(&self, n: Point) -> Point {
        Point {
            x: self.x + n.x,
            y: self.y + n.y,
        }
    }

    fn subtract(&self, n: Point) -> Point {
        Point {
            x: self.x - n.x,
            y: self.y - n.y,
        }
    }
    
}

impl Geometry for Point {
    fn describe(&self) -> String {
        String::from("point")
    }
}

