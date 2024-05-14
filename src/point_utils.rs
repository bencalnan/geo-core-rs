use crate::{
    constants::EARTH_RADIUS, latlng::LatLng, point::Point, rectangle::Rectangle, utils::deg_to_rad,
};

//PointToPointDistanceCosine Get distance (in meters) between two Lat/Lons using cosine formula. Has some benefits over using Haversine formula.
//Input and Output in Radians
pub fn point_to_point_distance_cosine(start_point: Point, end_point: Point) -> f64 {
    (start_point.y.sin() * end_point.y.sin()
        + start_point.y.cos() * end_point.y.cos() * (start_point.x - end_point.x).cos())
    .acos()
        * EARTH_RADIUS
}

// //PointToPointHaversine - Alternative method for getting distance (in meters) bewteen two Lat/Longs using cosine formula. Possibly slightly faster.
pub fn point_to_point_haversine(start: LatLng, end: LatLng) -> f64 {
    let start_radian = start.convert_to_radian();
    let end_radian = end.convert_to_radian();
    let dif_lat = end_radian.lat - start_radian.lat;
    let dif_lng = end_radian.lng - start_radian.lng;

    let a = (dif_lat / 2.0).sin() * (dif_lat / 2.0).sin()
        + start_radian.lat.cos()
            * end_radian.lat.cos()
            * (dif_lng / 2.0).sin()
            * (dif_lng / 2.0).sin();
    let c = 2.0 * (a.sqrt().atan2((1.0 - a).sqrt()));
    let d = EARTH_RADIUS * c;
    return d;
}

//PointToPointCartesianDistance - Get distance betweeen two cartesian points
pub fn point_to_point_cartesian_distance(source: Point, target: Point) -> f64 {
    ((target.x - source.x).powi(2) + (target.y - source.y).powi(2)).sqrt()
}

//https://stackoverflow.com/questions/5254838/calculating-distance-between-a-point-and-a-rectangular-box-nearest-point
//PointToRectangleDistance - Get distance to closet point on a rectangle
//Cartesian coordinates.
pub fn point_to_rectangle_distance(p: Point, r: Rectangle) -> f64 {
    let pre_max_x = (r.min_point.x - p.x).max(0.0);
    let dx = (pre_max_x).max(p.x - r.max_point.x);
    let pre_max_y = (r.min_point.y - p.y).max(0.0);
    let dy = (pre_max_y).max(p.y - r.max_point.y);
    return (dx * dx + dy * dy).sqrt();
}

//PointToRectangleDistance - Get distance to closet point on a rectangle/boudning box
// Geographic
// TODO Confirm this is working
pub fn point_to_rectangle_distance_geographic(l: LatLng, b: Rectangle) -> f64 {
    let converted = l.convert_to_radian();
    let c_as_point = converted.convert_to_point();

    if check_between_longs(&l, &b) {
        //If between Lngs then:
        //Either straight down or straight up to bounding box.
        if l.lat < b.min_point.y {
            return point_to_point_distance_cosine(
                c_as_point,
                Point {
                    x: c_as_point.x,
                    y: deg_to_rad(b.min_point.y),
                },
            );
        }
        if l.lat > b.max_point.y {
            return point_to_point_distance_cosine(
                c_as_point,
                Point {
                    x: c_as_point.x,
                    y: deg_to_rad(b.max_point.y),
                },
            );
        }
        //Actually inside so return 0 distance
        return 0.0;
    }

    //If on West
    if l.lng < b.min_point.x {
        //If North West, Use North West Corner
        if l.lat > b.max_point.y {
            return point_to_point_distance_cosine(
                c_as_point,
                Point {
                    x: deg_to_rad(b.min_point.x),
                    y: deg_to_rad(b.max_point.y),
                },
            );
        }
        //If South West, use South West Corner
        if l.lat < b.min_point.y {
            return point_to_point_distance_cosine(
                c_as_point,
                Point {
                    x: deg_to_rad(b.min_point.x),
                    y: deg_to_rad(b.min_point.y),
                },
            );
        }
        //Else Use line
        return point_to_point_distance_cosine(
            c_as_point,
            Point {
                x: deg_to_rad(b.min_point.x),
                y: c_as_point.y,
            },
        );
    }

    //must be on east
    //If North East, Use North East Corner
    if l.lat > b.max_point.y {
        return point_to_point_distance_cosine(
            c_as_point,
            Point {
                x: deg_to_rad(b.max_point.x),
                y: deg_to_rad(b.max_point.y),
            },
        );
    }
    //If South East, use South East Corner
    if l.lat < b.min_point.y {
        return point_to_point_distance_cosine(
            c_as_point,
            Point {
                x: deg_to_rad(b.max_point.x),
                y: deg_to_rad(b.min_point.y),
            },
        );
    }
    //Else Use line
    return point_to_point_distance_cosine(
        c_as_point,
        Point {
            x: deg_to_rad(b.max_point.x),
            y: c_as_point.y,
        },
    );
}

fn check_between_longs(l: &LatLng, b: &Rectangle) -> bool {
    if l.lng >= b.min_point.x && l.lng <= b.max_point.x {
        return true;
    }
    return false;
}

// MidPointGeographicCoordinates - Get the mid point of two points (geographic)
// Input and Output in Radians.
pub fn mid_point_geographic_coordinates(a: LatLng, b: LatLng) -> LatLng {
    let bx = b.lat.cos() * (b.lng - a.lng).cos();
    let by = b.lat.cos() * (b.lng - a.lng).sin();
    let lng_mid = a.lng + by.atan2(a.lat.cos() + bx); // ?? Check
    let lat_mid = (a.lat.sin() + b.lat.sin())
        .atan2(((a.lat.cos() + bx) * (a.lat.cos() + bx) + by * by).sqrt());

    LatLng {
        lng: lng_mid,
        lat: lat_mid,
    }
}

// FindBearing - Returns bearing between a start and end point
// Input and Output in Radians.
pub fn find_bearing(start_point: Point, end_point: Point) -> f64 {
    let y = (end_point.x - start_point.x).sin() * end_point.y.cos();
    let x = start_point.y.cos() * end_point.y.sin()
        - start_point.y.sin() * end_point.y.cos() * (end_point.x - start_point.x).cos();
    y.atan2(x)
}

#[cfg(test)]
mod tests {
    use crate::utils::deg_to_rad;

    use super::*;

    #[test]
    fn test_point_to_point_distance_cosine() {
        let start_lat = 51.27936;
        let start_lng = 1.07263;
        let end_lat = 51.27741;
        let end_lng = 1.07281;

        let start_point = Point {
            x: deg_to_rad(start_lng),
            y: deg_to_rad(start_lat),
        };

        let end_point = Point {
            x: deg_to_rad(end_lng),
            y: deg_to_rad(end_lat),
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

        let start_lat_lng = LatLng {
            lat: start_lat,
            lng: start_lng,
        };

        let end_lat_lng = LatLng {
            lat: end_lat,
            lng: end_lng,
        };

        let distance = point_to_point_haversine(start_lat_lng, end_lat_lng);

        assert_eq!(distance.round(), 217.0);
    }

    #[test]
    fn test_point_to_bounding_box_geographic() {
        let test_bbox = Rectangle {
            min_point: Point {
                x: 1.070678,
                y: 51.279336,
            },
            max_point: Point {
                x: 1.071720,
                y: 51.277559,
            },
        };

        // //Point which should return a distnace to a point on the line.
        let start_point = LatLng {
            lat: 51.27936,
            lng: 1.07263,
        };
        let distance = point_to_rectangle_distance_geographic(start_point, test_bbox);

        assert_eq!(distance.round(), 210.0);
    }
}
