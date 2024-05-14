use std::f64::consts::PI;

use crate::{constants::EARTH_RADIUS, latlng::LatLng, point::Point};

// DegToRad - Convert degrees to radians
pub fn deg_to_rad(angle: f64) -> f64 {
    angle * PI / 180.0
}

// RadToDeg - Convert radians to degrees
pub fn rad_to_deg(angle: f64) -> f64 {
    angle * 180.0 / PI
}

//PointToPointDistanceCosine Get distance (in meters) between two Lat/Lons using cosine formula. Has some benefits over using Haversine formula.
//Input and Output in Radians
pub fn point_to_point_distance_cosine(start_point: Point, end_point: Point) -> f64 {
    (start_point.y.sin() * end_point.y.sin() + start_point.y.cos() * end_point.y.cos() * (start_point.x - end_point.x).cos()).acos() * EARTH_RADIUS
}


// //PointToPointHaversine - Alternative method for getting distance (in meters) bewteen two Lat/Longs using cosine formula. Possibly slightly faster.
pub fn point_to_point_haversine(start: LatLng, end: LatLng) ->f64 {
	let start_radian = start.convert_to_radian();
	let end_radian = end.convert_to_radian();
	let dif_lat = end_radian.lat - start_radian.lat;
	let dif_lng = end_radian.lng - start_radian.lng;

	let a = (dif_lat/2.0).sin() * (dif_lat/2.0).sin() + start_radian.lat.cos() * end_radian.lat.cos()* (dif_lng / 2.0).sin()*(dif_lng/2.0).sin();
	let c = 2.0 * (a.sqrt().atan2((1.0-a).sqrt()));
	let d = EARTH_RADIUS * c;
	return d
}

//PointToPointCartesianDistance - Get distance betweeen two cartesian points
pub fn point_to_point_cartesian_distance(source: Point, target: Point) ->f64 {
	((target.x-source.x).powi(2) + (target.y-source.y).powi(2)).sqrt()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_to_point_distance_cosine() {
        let start_lat = 51.27936;
        let start_lng = 1.07263;
        let end_lat = 51.27741;
        let end_lng = 1.07281;

        let start_point = Point {
            x: deg_to_rad(start_lng),
            y: deg_to_rad(start_lat)
        };
    
        let end_point = Point{
            x: deg_to_rad(end_lng), 
            y: deg_to_rad(end_lat)
        };
    
        let distance = point_to_point_distance_cosine(start_point, end_point);
    
        assert_eq!(distance.round(), 217.0);
    
    }
    #[test]
    fn test_point_to_point_distance_haversine() {

        let start_lat = 51.27936;
        let start_lng = 1.07263;
        let end_lat = 51.27741;
        let end_lng = 1.07281;


	    let start_lat_lng = LatLng{
            lat: start_lat, 
            lng: start_lng
        };

	    let end_lat_lng = LatLng{
            lat: end_lat, 
            lng: end_lng
        };

	    let distance = point_to_point_haversine(start_lat_lng, end_lat_lng);

        assert_eq!(distance.round(), 217.0);

}

}




