pub fn examples() {
    let mut vec1 = vec![1, 2, 3];
    borrows_vec(&vec1);

    mutably_borrows_vec(&mut vec1);
    println!("vec1: {:?}", vec1);
}

fn borrows_vec(vec: &Vec<i32>) {
    println!("vec: {:?}", vec);
}

fn mutably_borrows_vec(vec: &mut Vec<i32>) {
    vec.push(10)
}