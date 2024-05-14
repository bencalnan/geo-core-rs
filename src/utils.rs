use std::f64::consts::PI;

use crate::constants::EARTH_RADIUS;

// DegToRad - Convert degrees to radians
pub fn deg_to_rad(angle: f64) -> f64 {
    angle * PI / 180.0
}

// RadToDeg - Convert radians to degrees
pub fn rad_to_deg(angle: f64) -> f64 {
    angle * 180.0 / PI
}

//FindCrossTrackDistance - Cross-track distance: Returns min distance between a great arc and another point (Lat longs).
//This is the distance between a point and great arc, not a segment
//Input and Output in Radians
pub fn find_cross_track_distance(distance_ac: f64, bearing_ac: f64, bearing_ab: f64) -> f64 {
    ((distance_ac / EARTH_RADIUS).sin() * (bearing_ac - bearing_ab).sin()).asin() * EARTH_RADIUS
}
