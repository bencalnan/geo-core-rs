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
            min_point: Point {
                x: min_x.unwrap(),
                y: min_y.unwrap(),
            },
            max_point: Point {
                x: max_x.unwrap(),
                y: max_y.unwrap(),
            },
        }
    }

    fn closed_chain(&self) -> bool {
        let start = self.lines[0].coords[0];
        let end = self.lines[self.lines.len() - 1].coords[1];
        let mut x = false;
        let mut y = false;
        if (start.x - end.x).abs() < 0.001 {
            x = true
        }
        if (start.y - end.y).abs() < 0.001 {
            //As not allowed to compare two f64s
            y = true
        }
        if x && y {
            return true;
        }
        false
    }

    fn centroid(&self) -> Point {
        let mut x_top: f64 = 0.0;
        let mut y_top: f64 = 0.0;
        let mut x_bottom: f64 = 0.0;
        let mut y_bottom: f64 = 0.0;

        for l in self.lines.iter() {
            let centroid = l.centroid();
            let length = l.length();
            x_top += centroid.x * length;
            y_top += centroid.y * length;
            x_bottom += length;
            y_bottom += length;
        }

        let x_centroid = x_top / x_bottom;
        let y_centroid = y_top / y_bottom;
        Point {
            x: x_centroid,
            y: y_centroid,
        }
    }
}

// Traits
impl Geometry for PolyLine {
    fn describe(&self) -> String {
        String::from("polyline")
    }
}
