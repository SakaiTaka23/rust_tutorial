fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let result = numbers
        .iter()
        .filter(|&&num| num % 2 != 0)
        .map(|&num| num * num)
        .sum::<i32>();

    println!("Result without combinators: {}", result);
}
