pub fn add(left: usize, right: usize) -> usize {
    left + right
}

mod shapes {
    pub struct Circle {
        radius: f32,
    }
    impl Circle {
        pub fn new(radius: f32) -> Circle {
            Circle { radius }
        }

        pub fn new_2(radius: f32) -> Circle {
            match radius {
                ..=0.0 => panic!("radius should be positive"),
                _ => Circle { radius },
            }
        }

        pub fn contains(&self, other: &Circle) -> bool {
            self.radius > other.radius
        }
    }
}

#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

#[cfg(test)]
mod shape_test {
    use super::shapes::*;

    #[test]
    fn larger_circle_should_contain_smaller() {
        let larger_circle = Circle::new(5.0);
        let smaller_circle = Circle::new(2.0);
        assert!(larger_circle.contains(&smaller_circle));
    }
    #[test]
    fn smaller_circle_should_not_contain_larger() {
        let larger_circle = Circle::new(5.0);
        let smaller_circle = Circle::new(2.0);
        assert_eq!(!smaller_circle.contains(&larger_circle), true, "custom error message");
        assert_ne!(larger_circle.contains(&smaller_circle), false);
    }

    #[test]
    #[should_panic]
    fn should_not_create_and_panic() {
        let some_circle = Circle::new_2(-5.0);
    }
}
