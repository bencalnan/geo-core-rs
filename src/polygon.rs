#![allow(dead_code)]

use crate::geometry::Geometry;
use crate::line::Line;
use crate::point::Point;
use crate::rectangle::Rectangle;
//Polygon - Closed Chain Polyline
struct Polygon {
    lines: Vec<Line>,
}

impl Polygon {
    fn new_from_points(points: Vec<Point>) -> Polygon {
        let mut pg = Polygon { lines: Vec::new() };
        for (i, pt) in points.iter().enumerate() {
            if i > 0 {
                let l: Line = Line {
                    coords: [points[i - 1], *pt],
                };
                pg.lines.push(l);
            }
        }

        // Final linestring to close end and first point.
        pg.lines
            .push(Line::new(points[points.len() - 1], points[0]));
        pg
    }
    //Vertices - Returns distinct vertices that make up the Polygon.
    fn vertices(&self) -> Vec<Point> {
        let mut distinct_points: Vec<Point> = Vec::new();
        for line in &self.lines {
            distinct_points.push(line.coords[0].clone());
        }
        distinct_points
    }

    // GetNumEdges - Returns NumEdges returns the number of edges in this shape.
    fn num_edges(&self) -> usize {
        if self.lines.len() == 0 {
            return 0;
        }
        return self.lines.len();
    }

    // Perimeter - Returns perimeter of polygon
    fn perimeter(&self) -> f64 {
        let mut d = 0.0;
        for line in &self.lines {
            d = d + line.length()
        }
        return d;
    }

    // Area - Returns area of polygon
    // https://www.mathopenref.com/coordpolygonarea.html
    // Note does not work for self intersecting polygons. (need to add catch for this. )
    fn area(&self) -> f64 {
        let mut distinct_points: Vec<Point> = self.vertices();
        distinct_points.push(distinct_points[0].clone());
        let mut sub_total: f64 = 0.0;
        for (i, _) in distinct_points.iter().enumerate() {
            let part: f64 = (distinct_points[i].x * distinct_points[i + 1].y)
                + (distinct_points[i].y * distinct_points[i + 1].x);
            sub_total = sub_total + part
        }
        return sub_total / 2.0;
    }

    fn bbox(&self) -> Rectangle {
        let distinct_points = &self.vertices().clone();
        let mut min_x: f64 = 0.0;
        let mut min_y: f64 = 0.0;
        let mut max_x: f64 = 0.0;
        let mut max_y: f64 = 0.0;
        for pt in distinct_points {
            if pt.x < min_x {
                min_x = pt.x
            }
            if pt.y < min_y {
                min_y = pt.y
            }
            if pt.x > max_x {
                max_x = pt.x
            }
            if pt.y > max_y {
                max_y = pt.y
            }
        }
        return Rectangle {
            p1: Point { x: min_x, y: min_y },
            p2: Point { x: max_x, y: max_y },
        };
    }

    //ClosedChain - Check if is a closed chain of lines (i.e. it is a Polygon)
    fn closed_chain(&self) -> bool {
        let start = self.lines[0].coords[0];
        let end = self.lines[self.lines.len() - 1].coords[1];
        let mut x = false;
        let mut y = false;
        if start.x == end.x {
            x = true
        }
        if start.y == end.y {
            y = true
        }
        if x == true && y == true {
            return true;
        }
        return false;
    }
}

// Traits
impl Geometry for Polygon {
    fn describe(&self) -> String {
        String::from("polygon")
    }
}
