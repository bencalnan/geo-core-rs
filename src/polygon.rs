#![allow(dead_code)]
use crate::line::Line;

//Polygon - Closed Chain Polyline
struct Polygon {
    lines: Vec<Line>,
}

impl Polygon {
    fn new_from_points(points: Vec<Point>) -> Polygon {
        let mut pg = Polygon { lines: Vec::new() };
        for (i, pg) in points.iter().enumerate() {
            if i > 0 {
                let l: Line = Line {
                    coords: [points[i - 1], *pt],
                };
                pg.lines.push(l);
            }
        }

        // Final linestring to close end and first point.
        pg.lines
            .push(Line.new(points[points.length() - 1], points[0]));
        pg
    }
    //Vertices - Returns distinct vertices that make up the Polygon.
    fn vertices(&self) -> Vec<Point> {
        let mut distinct_points: Vec<Point> = Vec::new();
        for line in self.lines {
            distinct_points.push(line[0]);
        }
        distinct_points
    }

    // GetNumEdges - Returns NumEdges returns the number of edges in this shape.
    fn num_edges(&self) -> i32 {
        if self.lines.len() == 0 {
            return 0;
        }
        return self.lines.len();
    }

    // //Perimeter - Returns perimeter of polygon
    fn perimter(&self) -> f64 {
        let mut d = 0.0;
        for line in p.lines {
            d = d + line.length()
        }
        return d;
    }
}



// // Area - Returns area of polygon
// // https://www.mathopenref.com/coordpolygonarea.html
// // Note does not work for self intersecting polygons. (need to add catch for this. )
// func (p Polygon) Area() float64 {
// 	distinctPoints := p.Vertices()
// 	distinctPoints[len(distinctPoints)] = distinctPoints[0]
// 	var subTotal float64
// 	for i := 0; i < len(distinctPoints)-1; i++ {
// 		part := (distinctPoints[i].X * distinctPoints[i+1].Y) + (distinctPoints[i].Y * distinctPoints[i+1].X)
// 		subTotal = subTotal + part
// 	}
// 	return subTotal / 2
// }

// func (p *Polygon) bbox() BoundingBox {
// 	points := p.Vertices()
// 	points[len(points)] = points[0]

// 	var minX float64
// 	var minY float64
// 	var maxX float64
// 	var maxY float64

// 	for _, pt := range points {
// 		if pt.X < minX {
// 			minX = pt.X
// 		}
// 		if pt.Y < minY {
// 			minY = pt.Y
// 		}
// 		if pt.X > maxX {
// 			maxX = pt.X
// 		}
// 		if pt.Y > maxY {
// 			maxY = pt.Y
// 		}

// 	}
// 	return BoundingBox{Point{X: minX, Y: minY}, Point{X: maxX, Y: maxY}}
// }

// //ClosedChain - Check if is a closed chain of lines (i.e. it is a Polygon)
// func (p Polygon) ClosedChain() bool {
// 	start := p[0][0]
// 	end := p[len(p)-1][1]
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

// Traits
impl Geometry for Polygon {
    fn describe(&self) -> String {
        String::from("polygon")
    }
}
