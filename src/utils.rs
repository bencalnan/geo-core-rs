use std::f64::consts::PI;

// DegToRad - Convert degrees to radians
pub fn deg_to_rad(angle: f64) -> f64 {
    angle * PI / 180.0
}

// RadToDeg - Convert radians to degrees
pub fn rad_to_deg(angle: f64) -> f64 {
    angle * 180.0 / PI
}
