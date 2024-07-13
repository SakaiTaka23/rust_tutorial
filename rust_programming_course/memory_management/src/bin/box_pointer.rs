#[derive(Debug)]
enum List {
    Cons(i32, Option<Box<List>>),
}

struct HugeData;

fn main() {
    let x = 0.625;
    let y = Box::new(x);
    let z = &x;

    let list = List::Cons(1, Some(Box::new(List::Cons(2, Some(Box::new(List::Cons(3, None)))))));
    println!("{:?}", list);

    let data_1 = HugeData;
    let data_2 = Box::new(HugeData);

    let data_3 = data_1;
    let data_4 = data_2;
}
