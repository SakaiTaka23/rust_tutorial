struct User {
    name: String,
    age: u8,
    salary: u32,
}

fn is_valid_user<V1, V2>(name: &str, age: u8, simple_validator: V1, advance_validator: V2) -> bool
where
    V1: Fn(&str) -> bool,
    V2: Fn(u8) -> bool,
{
    simple_validator(name) && advance_validator(age)
}

fn main() {
    let person_1 = User {
        name: String::from("someone"),
        age: 35,
        salary: 40_000,
    };

    let validate_user = |name: &str| name.len() != 0;
    let validate_user_advance = |age: u8| age >= 30;
    println!("User validity {}", validate_user(&person_1.name));
    println!("User validity {}", is_valid_user(&person_1.name, person_1.age, validate_user, validate_user_advance));
}
