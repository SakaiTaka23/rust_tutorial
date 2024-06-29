pub fn example() {
    let x = 5;
    println!("{x}");

    let mut y = 5;
    println!("{y}");

    {
        y = 10;
        println!("{y}");
        let y = 30;
        println!("{y}");
    }
    println!("{y}");
}
