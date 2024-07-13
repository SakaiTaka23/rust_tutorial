use std::cell::RefCell;

fn main() {
    let mut x = RefCell::new(50);
    let x1 = x.borrow();
    let x2 = x.borrow();
    drop(x1);
    drop(x2);
    let x3 = x.borrow_mut();
    println!("x3: {}", x3);
}
