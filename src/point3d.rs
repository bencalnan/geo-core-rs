#![allow(dead_code)]
use crate::constants::EARTH_RADIUS;
use crate::latlng::LatLng;
use crate::point::GeomType;

//Point3D - 3 Dimensional Point
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    fn new(x: f64, y: f64, z: f64) -> Point3D {
        Point3D { x: x, y: y, z: z }
    }
    //ConvertToLatLng - Takes a 3D Cartesian Point (ECEF) and returns Lat/Lngs in Radians. (Geocentric -> Geodetic)
    fn convert_to_lat_lng(&self) -> LatLng {
        LatLng {
            lat: (self.z / EARTH_RADIUS).asin(),
            lng: self.y.atan2(self.x),
        }
    }
}

impl GeomType for Point3D {
    fn describe(&self) -> String {
        String::from("point3D")
    }
}
