enum Operation<T> {
    Addition(T, T),
    Multiplication(T, T),
    Subtraction(T, T),
    Division(T, T),
}

impl Operation<i32> {
    fn exec(&self) -> i32 {
        match self {
            Operation::Addition(x, y) => x + y,
            Operation::Multiplication(x, y) => x * y,
            Operation::Subtraction(x, y) => x - y,
            Operation::Division(x, y) => x / y,
        }
    }
}

impl Operation<f64> {
    fn exec(&self) -> f64 {
        match self {
            Operation::Addition(x, y) => x + y,
            Operation::Multiplication(x, y) => x * y,
            Operation::Subtraction(x, y) => x - y,
            Operation::Division(x, y) => x / y,
        }
    }
}

fn main() {
    let op_1 = Operation::Addition(5, 10).exec();
    let op_2 = Operation::Multiplication(3.5, 2.0).exec();
    let op_3 = Operation::Subtraction(3.5, 2.0).exec();
    let op_4 = Operation::Division(2, 3).exec();

    println!("results: 5 + 10: {}, 3.5 * 2.0: {}, 3.5 - 2.0: {}, 2 / 3: {}", op_1, op_2, op_3, op_4)
}
