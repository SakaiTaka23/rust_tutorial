fn first_character(chars: &Vec<char>) -> Option<char> {
    if chars.len() > 0 {
        Some(chars[0])
    } else {
        None
    }
}

fn print_result(result: Option<char>) {
    match result {
        Some(character) => println!("First character: {character}"),
        None => println!("Empty array"),
    }
}

pub fn q1() {
    let my_chars = vec!['a', 'b', 'c', 'd'];
    print_result(first_character(&my_chars));
    let my_chars = vec![];
    print_result(first_character(&my_chars))
}

fn check_fruit(input_fruit: String) -> Option<String> {
    let fruit_basket = vec![
        String::from("mango"),
        String::from("apple"),
        String::from("banana"),
    ];
    for fruit in fruit_basket {
        if input_fruit == fruit {
            return Some(fruit);
        }
    }
    None
}

pub fn q2() {
    let user_fruit = String::from("apple");
    if let Some(fruit) = check_fruit(user_fruit) {
        println!("User's name: {fruit}");
    }
}
