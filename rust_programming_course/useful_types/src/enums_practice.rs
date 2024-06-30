enum Value {
    Integer(i32),
    Float(f32),
}

pub fn q1() {
    let some_val = vec![Value::Integer(12), Value::Float(15.5)];

    for i in some_val {
        match i {
            Value::Integer(num) => println!("Integer: {} ", num),
            Value::Float(num) => println!("Float: {}", num),
        }
    }
}

struct Item {
    id: u32,
    title: String,
    year: u16,
    item_type: ItemType,
}

impl Item {
    fn display_item_info(&self) {
        println!("id: {}, title: {}, year: {}, item_type: {:?}", self.id, self.title, self.year, self.item_type);
    }
}

#[derive(Debug)]
enum ItemType {
    Book,
    Magazine,
}

pub fn q2() {
    let book = Item {
        id: 1,
        title: String::from("The Rust Programming Language"),
        year: 2018,
        item_type: ItemType::Book,
    };
    book.display_item_info();
}
