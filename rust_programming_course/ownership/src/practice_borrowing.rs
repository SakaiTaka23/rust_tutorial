pub fn q1() {
    let some_vec = vec![1, 2, 3];
    let first = get_first_element(&some_vec);
    println!("first = {}", first);
    let _a = some_vec[0];
}

fn get_first_element(num_vec: &Vec<i32>) -> &i32 {
    &num_vec[0]
}

pub fn q2() {
    let vec_1 = vec![1, 2, 3];
    let vec_2 = vec![4, 5, 6];
    let mut vec_ptr: &Vec<i32>;
    vec_ptr = &vec_1;
    println!("vec ptr is pointing to vec_1 {:?}", vec_ptr);
    vec_ptr = &vec_2;
    println!("vec ptr is updated and now pointing to vec_2 {:?}", vec_ptr);
}

pub fn q3() {
    let mut first_num = 42;
    let mut second_num = 64;
    let ref1 = &mut first_num;
    let mut ref2 = &mut second_num;

    *ref1 = 15;
    *ref2 = 10;
    ref2 = ref1;

    println!("updated first number: {ref2}")
}
