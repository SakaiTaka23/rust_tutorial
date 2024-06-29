pub fn q1() {
    let x: u8;
    x = 1;
    println!("x is: {}", x);
}

pub fn q2() {
    let pi: f64;
    pi = std::f64::consts::PI;
    println!("pi is: {}", pi);
}

pub fn q3() {
    let a: i32 = -15;
    let b: i32 = 170;
    let name: &str = "Michael";
    println!("name is: {}, and the multiplication result is {}", name, a * b);
}

pub fn q4() {
    type Book = (String, String, u32);

    let book1: Book = (
        String::from("Rust Programming Langauge"),
        String::from("RUST Community"),
        2010,
    );
    println!(
        "Book name: {}, Author: {}, Year {}",
        book1.0, book1.1, book1.2
    );

    let book2: Book = (
        String::from("Rust by Example"),
        String::from("Steve Klabnik and Carol Nichols"),
        2015,
    );
    println!(
        "Book name: {}, Authors: {}, Year {}",
        book2.0, book2.1, book2.2
    );
}
