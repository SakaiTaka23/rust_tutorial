pub fn example() {
    let fixed_str = "fixed string";
    // error even with mut
    // fixed_str.push_str(" with push_str");
    println!("{fixed_str}");
    let mut str = String::from("string");
    str.push_str(" with push_str");
    println!("{str}");

    let array_1 = [1, 2, 3, 4];
    let array_1 = [array_1, array_1].join(&0);
    println!("{:?}", array_1);
}
