use std::cell::RefCell;

fn main() {
    let data: RefCell<Option<i32>> = RefCell::new(Some(42));

    let mut data2 = data.borrow_mut();
    *data2 = None;

    if data2.is_some() {
        println!("Final value: {:?}", data.borrow());
    } else {
        println!("No value present.");
    }
}
