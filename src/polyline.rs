#![allow(dead_code)]
use crate::line::Line;
use crate::point::GeomType;
use crate::point::Point;

//PolyLine - Aka Polygonal chain, linestring,
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
}

// // Creates a Polyline from a slice of Points
// func createPolylineFromPoints(points []Point) PolyLine {
// 	var p PolyLine
// 	for i, pt := range points {
// 		if i > 0 {
// 			line := createLine(points[i-1], pt)
// 			p = append(p, line)
// 		}
// 	}
// 	return p
// }

// // Creates a Polyline from a slice of lines
// func createPolyLineFromLines(lines []Line) PolyLine {
// 	var p PolyLine
// 	for _, l := range lines {
// 		p = append(p, l)
// 	}
// 	return p
// }

// //GetLength - Returns length of a polyline.
// func (p *PolyLine) length() float64 {
// 	var d float64
// 	for _, l := range *p {
// 		d = d + l.length()
// 	}
// 	return d
// }

// // GetVertices - returns all vertices in Polyline.
// func (p *PolyLine) vertices() []Point {
// 	var v []Point
// 	for i, l := range *p {
// 		if i == 0 {
// 			v = append(v, Point{X: l[0].X, Y: l[0].Y}) // Add first point as well on first line.
// 		}
// 		v = append(v, Point{X: l[1].X, Y: l[1].Y}) // Only add 2nd Point normally so don't duplicate.
// 	}
// 	return v
// }

// func (p *PolyLine) getNumVertices() int {
// 	return len(*p) + 1
// }

// func (p *PolyLine) bbox() BoundingBox {
// 	points := p.vertices()
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

// // NumEdges returns the number of edges in this shape. // Copied from S2 //Move out, and create interface.
// func (p *PolyLine) getNumEdges() int {
// 	if len(*p) == 0 {
// 		return 0
// 	}
// 	return len(*p) - 1
// }

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
impl GeomType for PolyLine {
    fn describe(&self) -> String {
        String::from("polyline")
    }
}
