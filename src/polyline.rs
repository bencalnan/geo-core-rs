#![allow(dead_code)]
use crate::geometry::Geometry;
use crate::line::Line;
use crate::point::Point;
use crate::rectangle::Rectangle;

struct PolyLine {
    lines: Vec<Line>,
}
impl PolyLine {
    fn new_from_points(points: Vec<Point>) -> PolyLine {
        let mut pl = PolyLine { lines: Vec::new() };
        for (i, pt) in points.iter().enumerate() {
            if i > 0 {
                let l: Line = Line {
                    coords: [points[i - 1], *pt],
                };
                pl.lines.push(l);
            }
        }
        pl
    }
    fn new_from_lines(lines: Vec<Line>) -> PolyLine {
        PolyLine {
            lines: lines.to_vec(),
        }
    }
    fn length(&self) -> f64 {
        let mut length = 0.0;
        for l in self.lines.iter() {
            length += l.length()
        }
        length
    }

    fn vertices(&self) -> Vec<Point> {
        let mut v = Vec::<Point>::new();
        for (i, l) in self.lines.iter().enumerate() {
            if i == 0 {
                v.push(l.coords[0]) // Add first point as well on first line.
            }
            v.push(l.coords[1]) // Only add 2nd Point normally so don't duplicate.
        }
        v
    }

    fn num_vertices(&self) -> usize {
        self.lines.len() + 1
    }

    fn edges(&self) -> Vec<Line> {
        self.lines.to_vec()
    }

    fn num_edges(&self) -> usize {
        self.lines.len()
    }

    fn bbox(&self) -> Rectangle {
        let mut min_x: Option<f64> = None;
        let mut min_y: Option<f64> = None;
        let mut max_x: Option<f64> = None;
        let mut max_y: Option<f64> = None;
        let vs = self.vertices();
        for p in vs.iter() {
            match min_x {
                Some(current_min_x) => {
                    if p.x < current_min_x {
                        min_x = Some(p.x)
                    }
                }
                None => min_x = Some(p.x),
            }
            match min_y {
                Some(current_min_y) => {
                    if p.y < current_min_y {
                        min_y = Some(p.y)
                    }
                }
                None => min_y = Some(p.y),
            }

            match max_x {
                Some(current_max_x) => {
                    if p.x > current_max_x {
                        max_x = Some(p.x)
                    }
                }
                None => max_x = Some(p.x),
            }

            match max_y {
                Some(current_max_y) => {
                    if p.y > current_max_y {
                        max_y = Some(p.y)
                    }
                }
                None => max_y = Some(p.y),
            }
        }

        Rectangle {
            p1: Point {
                x: min_x.unwrap(),
                y: min_y.unwrap(),
            },
            p2: Point {
                x: max_x.unwrap(),
                y: max_y.unwrap(),
            },
        }
    }
}


// //ClosedChain - Check if is a closed chain of lines (i.e. it is a Polygon)
// func (p *PolyLine) checkClosedChain() bool {
// 	pV := *p
// 	start := pV[0][0]
// 	end := pV[len(pV)-1][1]
// 	x, y := false, false
// 	if start.X == end.X {
// 		x = true
// 	}
// 	if start.Y == end.Y {
// 		y = true
// 	}
// 	if x == true && y == true {
// 		return true
// 	}
// 	return false

// }

// //centroid - Return centroid of a polyline
// func (p *PolyLine) centroid() Point {

// 	var xTop = 0.0
// 	var yTop = 0.0
// 	var xBottom = 0.0
// 	var yBottom = 0.0
// 	for _, l := range *p {
// 		centroid := l.centroid()
// 		length := l.length()
// 		xTop = xTop + centroid.X*length
// 		yTop = yTop + centroid.Y*length
// 		xBottom = xBottom + length
// 		yBottom = yBottom + length
// 	}
// 	xCentroid := xTop / xBottom
// 	yCentroid := yTop / yBottom
// 	return Point{X: xCentroid, Y: yCentroid}
// }

// Traits
impl Geometry for PolyLine {
    fn describe(&self) -> String {
        String::from("polyline")
    }
}
