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