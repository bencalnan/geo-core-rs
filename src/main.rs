mod point;
mod point3d;
mod latlng;
mod utils;
mod constants; 
mod rectangle;
mod line;
mod polyline;
mod geometry; 

use crate::point::Point;

fn main() {
    let p = Point{x: 5.3, y: 4.3};
    println!("X: {}", p.x);
    
}
