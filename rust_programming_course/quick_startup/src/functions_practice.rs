pub fn q1() {
    let x = 3;
    let y = 4;
    println!(
        "The result of x+3 times y+5 is {}",
        times(add_3(x), add_5(y))
    );
}

fn add_3(x: i32) -> i32 {
    x + 3
}

fn add_5(x: i32) -> i32 {
    x + 5
}

fn times(x: i32, y: i32) -> i32 {
    x * y
}

pub fn q2() {
    let x = triple(double(5));
    let y = triple(x);
    println!("Answer: {}", y);
}

fn double(x: i32) -> i32 {
    x * 2
}

fn triple(x: i32) -> i32 {
    x * 3
}

pub fn q3() {
    println!(
        "The distance of the number the point from the origin is {}",
        print_distance((5.0, 4.0))
    );
}

fn print_distance(point: (f32, f32)) -> f32 {
    let (x, y) = point;
    (x.powf(2.0) + y.powf(2.0)).sqrt() // Formula for computing distance
}

pub fn q4() {
    println!("For 1: the expected value is 4 while the output is {}", quadruple(1));
}

fn quadruple(x: i32) -> i32 {
    double(x) * 2
}
