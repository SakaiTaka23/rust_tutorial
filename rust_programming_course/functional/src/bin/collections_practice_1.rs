fn main() {
    let mut vec_1 = vec![4, 5, 6, 9, 8];
    for i in &mut vec_1.iter_mut() {
        *i = *i * 2;
    }
    println!("{:?}", vec_1);
}
