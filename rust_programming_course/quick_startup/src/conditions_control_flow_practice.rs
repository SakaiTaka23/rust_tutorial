use rand::Rng;

pub fn q1() {
    let n = rand::thread_rng().gen_range(1..=100);

    let mut square_of_sum = 0;
    let mut sum_of_squares = 0;
    for n in 1..=n {
        square_of_sum += n;
        sum_of_squares += n * n;
    }
    square_of_sum *= square_of_sum;
    let diff = square_of_sum - sum_of_squares;

    println!("input {}, square of sum {}, sum of squares {} difference {}", n, square_of_sum, sum_of_squares, diff);
}

pub fn q2() {
    let n = rand::thread_rng().gen_range(1..=100);
    let mut result = 0;

    for n in 1..=n {
        if n % 15 == 0 {
            result += n
        } else if n % 5 == 0 {
            result += n
        } else if n % 3 == 0 {
            result += n
        }
    };

    println!("input {}, result {}", n, result);
}

pub fn q3() {
    println!("{}", total_production(6, 5) as i32);
    println!("{}", cars_produced_per_minutes(6, 5) as i32);
}

fn total_production(hours: u8, speed: u8) -> f32 {
    hours as f32 * speed as f32 * 221f32
}

fn cars_produced_per_minutes(hours: u8, speed: u8) -> f32 {
    let success_rate = match speed {
        1..=4 => 100.0,
        5..=8 => 90.0,
        9..=10 => 77.0,
        _ => panic!("Invalid speed: {}", speed)
    };

    total_production(hours, speed) * (success_rate / 100.0) / (60.0 * hours as f32)
}

pub fn q4() {
    let input = String::from("1211");
    println!("It is {:?} that the given string {} is palindrome", palindrome(&input), input);

    let input = String::from("しんぶんし");
    println!("It is {:?} that the given string {} is palindrome", palindrome(&input), input);
}

fn palindrome(input: &String) -> bool {
    let mut reversed = String::new();

    for i in input.chars().rev() {
        reversed.push(i);
    }
    println!("reversed: {}", reversed);

    *input == reversed
}

pub fn q5() {
    'outer: for ia in 1..999 {
        for ib in ia + 1..500 {
            for ic in ib + 1..997 {
                if ia + ib + ic != 1000 || ia * ia + ib * ib != ic * ic {
                    continue;
                }
                println!("found: {}, {}, {}", ia, ib, ic);
                break 'outer;
            }
        }
    }
}

pub fn q6() {
    assert_eq!(can_see_movie(20, false), true);
    assert_eq!(can_see_movie(15, true), true);
    assert_eq!(can_see_movie(15, false), false);
}

fn can_see_movie(age: i32, permission: bool) -> bool {
    age >= 17 || (age >= 13 && permission)
}
