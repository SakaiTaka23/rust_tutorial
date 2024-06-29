mod basics;
mod functions;
mod practice_ownership;
mod borrowing;
mod borrowing_functions;
mod practice_borrowing;
mod dereferencing;

fn main() {
    println!("basics");
    basics::examples();

    println!("functions");
    functions::examples();

    println!("practice ownership");
    println!("q1");
    practice_ownership::q1();
    println!("q2");
    practice_ownership::q2();
    println!("q3");
    practice_ownership::q3();

    println!("borrowing");
    borrowing::examples();

    println!("borrowing functions");
    borrowing_functions::examples();

    println!("practice borrowing");
    println!("q1");
    practice_borrowing::q1();
    println!("q2");
    practice_borrowing::q2();
    println!("q3");
    practice_borrowing::q3();

    println!("dereferencing");
    dereferencing::examples();
}
