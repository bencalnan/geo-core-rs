![Rust](https://github.com/bencalnan/geo-core-rs/workflows/Rust/badge.svg)

# geo-core-rs
Library of common geographic functions and tools in Rust


### Point
 - Create points from XY
 - Create point from Lat Lng
 - Add Point
 - Subtract point

 ### Line
 - Create line from 2 points
 - Get Length
 - Get Centroid
 - Get Bounding Box

 ### Polyline

- Create polyline from points
- Create polyline from lines
- Get Length
- Get Centroid
- Get Vertices (Array of Points)
- Get Number of vertices
- Get Bounding Box
- Get Edges (Array of Lines)
- Get Number of edges
- Check if closed chain of lines (i.e. a polygon)


### Polygon

- Create Polygon from points
- Get Vertices (Array of Points)
- Get Number of Edges
- Get Perimeter (length)
- Get Area
- Get Bounding Box
- Check if closed chain. 


### Utilities

- Convert degrees to radians
- Convert radians to degrees
- Point To Point Distance Cosine (Geographic)
- Point To Point Distance Haversine (Geographic)
- Point To Point Distance  (Cartesian)
- Point to Rectangle Distance  (Cartesian)
- Point to Rectangle Distance (Geographic)
- Point to Line Distance (Cartesian)
- Point to Line Distance (Geographic)
- Mid point of two points (Geographic)
- Find bearing between start and end point (Geographic)
- Find cross track min distance between a great arc and another point (Geographic)