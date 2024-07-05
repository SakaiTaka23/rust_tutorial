fn find_max(nums: &[i32]) -> i32 {
    if nums.is_empty() {
        panic!("Empty number list");
    }
    let mut max = nums[0];
    for &num in nums {
        if num > max {
            max = num;
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Empty number list")]
    fn panics_on_empty_list() {
        let nums = [];
        let _max = find_max(&nums);
    }
}

fn main() {}
