pub fn examples() {
    let s1 = String::from("world");
    let s2 = s1.clone();
    println!("s1 is {}", s1);

    let _ = (s1.clone(), s2.clone());
    println!("s1 is {}", s1);

    let x = 10;
    let y = x;
    println!("x is {}, y is {}", x, y);
}
