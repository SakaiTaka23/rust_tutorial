fn main() {
    let mut counter = 0;

    let mut increment_counter = || counter += 1;
    increment_counter();
    increment_counter();
    outer_fn(&mut increment_counter);

    println!("Final Counter: {}", counter);
}

fn outer_fn<V1>(closure: &mut V1)
where
    V1: FnMut(),
{
    closure()
}
