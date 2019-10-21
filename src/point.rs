#![allow(dead_code)]
use crate::latlng::LatLng;
use std::ops::Add;
use std::ops::Sub;

#[derive(Copy, Clone)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl <T>Point<T> {
    fn new<T>(x: T, y: T) -> Point<T> {
        Point { x: x, y: y }
    }

    fn new_lat_lng(lat_lng: LatLng) -> Point<f64> {
        Point {
            x: lat_lng.lng,
            y: lat_lng.lat,
        }
    }

}

//Traits
impl <T>GeomType for Point<T> {
    fn describe(&self) -> String {
        String::from("point")
    }
}
impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}


pub trait GeomType {
    fn describe(&self) -> String;
}
