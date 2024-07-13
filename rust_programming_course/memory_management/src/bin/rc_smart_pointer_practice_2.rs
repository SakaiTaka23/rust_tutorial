use std::rc::Rc;

struct File {}

struct User {
    file: Rc<File>,
}

fn main() {
    let txt_file = Rc::new(File {});

    let _user_1 = User {
        file: Rc::clone(&txt_file),
    };

    let _user_2 = User {
        file: Rc::clone(&txt_file),
    };
}
