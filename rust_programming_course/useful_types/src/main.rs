mod structs;
mod structs_practice;
mod adding_functionality;
mod enums;
mod enums_practice;
mod option;
mod options_practice;
mod result;
mod result_practice;
mod hashmap;
mod hashmap_practice;

fn main() {
    println!("struct");
    structs::examples();

    println!("struct practice");
    println!("q1");
    structs_practice::q1();
    println!("q2");
    structs_practice::q2();

    println!("adding functionality");
    adding_functionality::examples();

    println!("enums");
    enums::examples();

    println!("enums practice");
    println!("q1");
    enums_practice::q1();
    println!("q2");
    enums_practice::q2();

    println!("option");
    option::examples();

    println!("options practice");
    println!("q1");
    options_practice::q1();
    println!("q2");
    options_practice::q2();

    println!("result");
    result::examples();

    println!("result practice");
    println!("q1");
    result_practice::q1();
    println!("q2");
    result_practice::q2();

    println!("hashmap");
    hashmap::examples();

    println!("hashmap practice");
    println!("q1");
    hashmap_practice::q1();
    println!("q2");
    hashmap_practice::q2();
}
