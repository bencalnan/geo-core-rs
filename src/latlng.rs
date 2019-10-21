#![allow(dead_code)]
use crate::point::Point;
use crate::point3d::Point3D;
use crate::utils::deg_to_rad;
use crate::utils::rad_to_deg;
use crate::constants::EARTH_RADIUS;

//LatLng - Lat Long Coordinate
pub struct LatLng {
    pub lat: f64,
    pub lng: f64,
    // pub alt: i32, //Optional Altitude
}

// Methods
impl LatLng {
    fn new(lat: f64, lng: f64) -> LatLng {
        LatLng { lat: lat, lng: lng }
    }
    fn new_from_point(p: Point<f64>) -> LatLng {
        LatLng { lat: p.y, lng: p.x }
    }
    fn convert_to_radian(&self) -> LatLng {
        LatLng {
            lat: deg_to_rad(self.lat),
            lng: deg_to_rad(self.lng),
        }
    }
    fn convert_to_degrees(&self) -> LatLng {
        LatLng {
            lat: rad_to_deg(self.lat),
            lng: rad_to_deg(self.lng),
        }
    }
    fn convert_to_point(&self) -> Point<f64> {
        Point {
            x: self.lng,
            y: self.lat,
        }
    }
    fn convert_to_point3d(&self) -> Point3D{
        Point3D {
            x: EARTH_RADIUS * self.lat.cos() * self.lng.cos(),
            y: EARTH_RADIUS * self.lat.cos() * self.lng.sin(),
            z: EARTH_RADIUS * self.lat.sin()
        }
    }
}
