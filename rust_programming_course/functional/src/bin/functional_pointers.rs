struct User {
    name: String,
    age: u8,
    salary: u32,
}

fn is_valid_user(
    name: &str,
    age: u8,
    simple_validator: fn(&str) -> bool,
    advance_validator: fn(u8) -> bool,
) -> bool
{
    simple_validator(name) && advance_validator(age)
}

fn validate_user_simple(name: &str) -> bool {
    name.len() != 0
}

fn validate_user_advance(age: u8) -> bool {
    age >= 30
}

fn main() {
    let person_1 = User {
        name: String::from("someone"),
        age: 35,
        salary: 40_000,
    };

    // let validate_user_simple = |name: &str| name.len() != 0;
    // let validate_user_advance = |age: u8| age >= 30;
    println!("User validity {}", validate_user_simple(&person_1.name));
    println!("User validity {}", is_valid_user(&person_1.name, person_1.age, validate_user_simple, validate_user_advance));
}
