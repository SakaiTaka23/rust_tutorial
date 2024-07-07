fn main() {
    let x = 10;
    let add_to_x = |y| y + x;

    let result = add_to_x(5);
    println!("Result: {}", result);
    println!("x {}", x);
}
