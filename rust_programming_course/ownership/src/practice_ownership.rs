pub fn q1() {
    let s1 = String::from("this is me, ");
    let s2 = "Nouman";

    some_function(s1.clone(), s2);
    println!("{} {}", s1, s2);
}

fn some_function(a1: String, a2: &str) {
    println!("{} {}", a1, a2);
}

pub fn q2() {
    let mut my_vec = vec![1, 2, 3, 4, 5];

    while !my_vec.is_empty() {
        let temp = my_vec.clone();
        println!("Elements in my_vec: {:?}", temp);

        if let Some(last_element) = my_vec.pop() {
            println!("Popped element: {}", last_element);
        }
    }
}

pub fn q3() {
    let str1 = generate_string();
    let str2 = str1;

    println!("{}", str2);
}

fn generate_string() -> String {
    String::from("I will generate a string")
}
