use std::fmt;
use std::fmt::{DebugStruct, Formatter};

trait Add<T> {
    fn add(&self, other: T) -> i32;
}

impl Add<String> for String {
    fn add(&self, other: String) -> i32 {
        let my_int: i32 = self.parse().unwrap();
        let other_int: i32 = other.parse().unwrap();
        my_int + other_int
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

struct Point2 {
    x: i32,
    y: i32,
}

impl fmt::Debug for Point2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Point [{} {}]", self.x, self.y)
    }
}

fn main() {
    let one = String::from("10");
    let two = String::from("35");
    let result = one.add(two);
    assert_eq!(result, 45);
    let point1 = Point { x: 0, y: 0 };
    assert_eq!(
        format!("The origin is: {:?}", point1),
        "The origin is: Point { x: 0, y: 0 }"
    );
    let point2 = Point2 { x: 1, y: 5 };
    assert_eq!(
        format!("The origin is: {:?}", point2),
        "The origin is: Point [1 5]"
    );
}
