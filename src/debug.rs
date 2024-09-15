// #[derive(Debug)]
// struct Point {
//     x: i32,
//     y: i32,
// }



// fn main() {
//     let point = Point { x: 5, y: 10 };
//     println!("The point is: {:?}", point);
// }

use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Le point {{ x: {}, y: {} }}", self.x, self.y)
    }
}

fn main() {
    let point = Point { x: 5, y: 10 };
    println!("The point is: {:?}", point);
}