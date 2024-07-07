struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Point<T, U> {
        Point { x, y }
    }
}

impl Point<i32, i32> {
    fn printing(&self) {
        println!(
            "The values of the coordinates are {} and {}",
            self.x, self.y
        );
    }
}

fn add_points<T, U>(p1: &Point<T, U>, p2: &Point<T, U>) -> Point<T, U> {
    todo!()
}

fn main() {
    let origin = Point { x: 0, y: 0 };
    origin.printing();
    let p1 = Point { x: 1.0, y: 4.0 };

    let p2 = Point::new(5, 5.0);
}
