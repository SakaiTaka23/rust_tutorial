struct ArrayProcessor<'a> {
    data: &'a [i32],
}

impl<'a> ArrayProcessor<'a> {
    fn update_data(&mut self, new_data: &'a [i32]) -> &[i32] {
        let previous_data = self.data;
        self.data = new_data;
        previous_data
    }
}

fn main() {
    let mut some_data = ArrayProcessor {
        data: &[4, 5, 6]
    };
}
