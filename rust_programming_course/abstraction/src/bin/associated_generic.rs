trait Addtion {
    type Rhs;
    type Output;
    fn add(self, rhs: Self::Rhs) -> Self::Output;
}

struct Point {
    x: i32,
    y: i32,
}

impl Addtion for Point {
    type Rhs = Point;
    type Output = Point;

    fn add(self, rhs: Self::Rhs) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: 2, y: 2 };
    let p3 = p1.add(p2);

    assert_eq!(p3.x, 3);
    assert_eq!(p3.y, 3);
}
