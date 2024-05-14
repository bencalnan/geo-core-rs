use crate::{constants::EARTH_RADIUS, latlng::LatLng, line::Line, point::Point, point3d::Point3D, point_utils::point_to_point_distance_cosine, utils::deg_to_rad};


// PointToLineCartesianDistance - Get the min distance from a point to a line (Cartesian Coordinates)
// https://en.m.wikipedia.org/wiki/Distance_from_a_point_to_a_line
pub fn point_to_line_cartesian_distance(p: Point, l: Line) -> f64 {
	let top = (((l.coords[1].y - l.coords[0].y) * p.x) - ((l.coords[1].x - l.coords[0].x) * p.y) + (l.coords[1].x * l.coords[0].y) - (l.coords[1].y * l.coords[0].x)).abs();
	let bottom = (((l.coords[1].y - l.coords[1].y) * 2.0) + (l.coords[1].x - l.coords[0].x*2.0)).sqrt();
	let result = top / bottom;
	return result
}

// PointToLineGeographicDistance - Gets the min distnace from a point to a line (Geographic Cordinates)
pub fn point_to_line_geographic_distance(p: LatLng, l: Line) -> f64 {
	let a = LatLng{
		lat: deg_to_rad(l.coords[0].y),
		lng: deg_to_rad(l.coords[0].x),
	};
	let b = LatLng{
		lat: deg_to_rad(l.coords[1].y),
		lng: deg_to_rad(l.coords[1].x),
	};
	let c = p.convert_to_radian();

	let t = nearest_point_great_circle(&a, &b, &c);
	let a_point = a.convert_to_point();
	let b_point = b.convert_to_point();
	let c_point = c.convert_to_point();
	let t_point = t.convert_to_point();

	//If closest point is on the line use that.
	if on_segment(a_point, b_point, t_point) {
		return point_to_point_distance_cosine(t_point, c_point)
	}

	//Otherwise just use start or end point whichever is closer.
	let distance_ac = point_to_point_distance_cosine(a_point, c_point);
	let distance_bc = point_to_point_distance_cosine(b_point, c_point);

	if distance_ac < distance_bc {
		return distance_ac
	}
	return distance_bc

}

fn nearest_point_great_circle(a: &LatLng, b: &LatLng, c: &LatLng) ->LatLng {
	let a_cartesian = a.convert_to_point3d();
	let b_cartesian = b.convert_to_point3d();
	let c_cartesian = c.convert_to_point3d();

	let g = vector_product(&a_cartesian, &b_cartesian);
	let f: Point3D = vector_product(&c_cartesian, &g);
	let t = vector_product(&g, &f);
	let norm = normalize(t);
	let multi = multiply_by_scalar(norm, EARTH_RADIUS);
	let cart = multi.convert_to_lat_lng();

	return cart
}

fn vector_product(a: &Point3D, b: &Point3D) -> Point3D {
	let result = Point3D{
		x: a.y*b.z - a.z*b.y,
		y: a.z*b.x - a.x*b.z,
		z: a.x*b.y - a.y*b.x,
	};
	return result
}

fn normalize(t: Point3D) -> Point3D {
	let length = ((t.x * t.x) + (t.y * t.y) + (t.z * t.z)).sqrt();
	let result = Point3D{
		x: t.x / length,
		y: t.y / length,
		z: t.z / length,
	};
	return result
}

fn multiply_by_scalar(normalize: Point3D, k: f64) -> Point3D {
	let result = Point3D{
		x: normalize.x * k,
		y: normalize.y * k,
		z: normalize.z * k,
	};
	return result
}

//Needs Radians -- Checks if point is on the line by substracting distances to check if total lenght - two sub lenghts are near enough to be zero.
fn on_segment(a: Point, b: Point, t: Point) -> bool {
	let diff = (point_to_point_distance_cosine(a, b) - point_to_point_distance_cosine(a, t) - point_to_point_distance_cosine(b, t)).abs();
	if diff < 0.1 {
		return true
	}
	return false
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_point_to_line_geographic() {
	let test_line = Line::new(Point{
        x: 1.070678, 
        y: 51.279336
    }, Point{
        x: 1.071720, 
        y: 51.277559
    });
       

	//Point which should return a distnace to a point on the line.
	let start_point = LatLng{
        lat: 51.27936, 
        lng: 1.07263
    };
	let distance = point_to_line_geographic_distance(start_point, test_line);
    assert_eq!(distance.round(), 128.0);


	//Point which should return a distance to the line's start point.
	let start_point2 = LatLng{
            lat: 51.280345,
            lng: 1.070411
        };
	let distance2 = point_to_line_geographic_distance(start_point2, test_line);
    assert_eq!(distance2.round(), 114.0);

	//Point which should return a distance to the line's end point.
	let start_point3 = LatLng{
        lat: 51.277410, 
        lng: 1.072814
    };
	let distance3 = point_to_line_geographic_distance(start_point3, test_line);

    assert_eq!(distance3.round(), 78.0);

    }

}
