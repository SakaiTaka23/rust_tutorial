pub fn examples() {
    let mut some_data = 42;
    let ref_1 = &mut some_data;
    let deref_copy = *ref_1;
    *ref_1 = 13;
    println!("deref_copy = {} ref_1 = {}", deref_copy, ref_1);

    let mut heap_data = vec![5, 6, 7];
    let ref_1 = &mut heap_data;
    let _ = ref_1.clone();
}
