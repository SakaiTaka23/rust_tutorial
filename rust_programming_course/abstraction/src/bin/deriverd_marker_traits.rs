#[derive(Debug, PartialEq, Clone, Default)]
struct Student {
    name: String,
    age: u8,
    sex: char,
}
impl Properties for Student {}

trait Properties: PartialEq + Default + Clone {}

fn main() {
    let s_1 = Student {
        name: String::from("abc"),
        age: 35,
        sex: 'M',
    };

    let s_2 = Student {
        name: String::from("xyz"),
        age: 40,
        sex: 'M',
    };

    println!("Student: {:?}", s_1);
    println!("s_1 and s_2 are equal: {}", s_1 == s_2);
}
