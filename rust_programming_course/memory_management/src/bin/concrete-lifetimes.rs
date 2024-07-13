fn main() {
    let i = 5;
    let _j = i;
    println!("i: {i}");

    let str_1 = String::from("abc");
    let _str_2 = str_1;
    // println!("str_1: {str_1}"); error

    let i;
    {
        let j = 5;
        i = &j;
        println!("i: {i}");
    }
    // println!("i: {i}"); error
}
