use std::collections::HashMap;

pub fn examples() {
    let mut person: HashMap<&str, i32> = HashMap::new();
    person.insert("John", 32);
    person.insert("Jane", 31);
    person.insert("Jill", 30);

    println!("The age is {}", person.get("John").unwrap());

    for (name, age) in &person {
        println!("{}: {}", name, age);
    }
}
