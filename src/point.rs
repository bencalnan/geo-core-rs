#![allow(dead_code)]

pub struct Point {
    pub x: f64,
    pub y: f64,
}

// Methods
impl Point {
    // Another static method, taking two arguments:
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
    //Instance Method
    fn add(&self, n: Point) -> Point {
        Point {
            x: self.x + n.x,
            y: self.y + n.y,
        }
    }

    //Subtract one point from another.
    fn subtract(&self, n: Point) -> Point {
        Point {
            x: self.x - n.x,
            y: self.y - n.y,
        }
    }
}

// Traits
impl GeomType for Point {
    fn describe(&self) -> String {
        String::from("point")
    }
}

pub trait GeomType {
    fn describe(&self) -> String;
}
