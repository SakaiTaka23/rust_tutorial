use crate::person::Student;

mod person {
    #[derive(Debug)]
    pub struct Student {
        id: u8,
        pub age: u8,
        pub name: String,
    }

    impl Student {
        pub fn new(std_name: String) -> Result<Self, String> {
            if std_name.chars().all(|x| matches!(x, 'a'..='z')) {
                Ok(
                    Self {
                        id: 0,
                        age: 20,
                        name: std_name,
                    }
                )
            } else {
                Err(String::from("invalid name"))
            }
        }
    }

    impl Default for Student {
        fn default() -> Self {
            Self {
                id: 0,
                age: 20,
                name: String::from("Unknown"),
            }
        }
    }
}

fn main() {
    let student_1 = Student::new(String::from("alex")).unwrap();
    println!("{:?}", student_1);

    let student_2 = Student::new(String::from("alex12344")).unwrap_or_default();
    println!("{:?}", student_2);
}
