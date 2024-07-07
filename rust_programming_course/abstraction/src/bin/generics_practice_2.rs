struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn create(value: T) -> Container<T> {
        Container { value }
    }
}

impl Container<i32> {
    fn display(&self) {
        println!("The value inside the container is: {}", self.value);
    }
}

fn main() {}
