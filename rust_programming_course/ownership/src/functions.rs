pub fn examples() {
    let vec_1 = vec![1, 2, 3];
    takes_ownership(vec_1.clone());
    println!("vec_1: {:?}", vec_1);

    let vec_2 = gives_ownership();
    println!("vec_2: {:?}", vec_2);

    let vec_3 = takes_and_gives_ownership(vec_2);
    println!("vec_3: {:?}", vec_3);

    let var = 5;
    stack_function(var);
    println!("var: {}", var);
}

fn takes_ownership(vec: Vec<i32>) {
    println!("vec: {:?}", vec);
}

fn gives_ownership() -> Vec<i32> {
    vec![4, 5, 6]
}

fn takes_and_gives_ownership(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(10);
    vec
}

fn stack_function(mut var: i32) {
    var += 1;
    println!("stack function var: {}", var);
}
