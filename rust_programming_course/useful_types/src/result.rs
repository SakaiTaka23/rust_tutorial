struct Student {
    name: String,
    grade: Option<u32>,
}

fn get_grade(student_name: &String, student_db: &Vec<Student>) -> Option<u32> {
    for student in student_db {
        if student.name == *student_name {
            return student.grade;
        }
    }
    None
}

fn check_student(student_name: &String, student_db: &Vec<Student>) -> Result<(), String> {
    for student in student_db {
        if student.name == *student_name {
            return Ok(());
        }
    }
    Err(String::from("Student not found"))
}

fn check_and_return_student(student_name: &String, student_db: &Vec<Student>) -> Result<Option<u32>, String> {
    for student in student_db {
        if student.name == *student_name {
            return Ok(student.grade);
        }
    }
    Err(String::from("Student not found"))
}

pub fn examples() {
    let student_db = vec![
        Student { name: String::from("Alice"), grade: Some(90) },
        Student { name: String::from("Bob"), grade: Some(80) },
        Student { name: String::from("Charlie"), grade: None },
    ];

    let student_name = String::from("Alice");
    let student_status = check_student(&student_name, &student_db);
    match student_status {
        Ok(_) => {
            let student_grade = get_grade(&String::from("Alice"), &student_db);
            match student_grade {
                Some(grade) => println!("Grade is {}", grade),
                None => println!("Student not found"),
            }

            if let Some(grade) = student_grade {
                println!("Grade is {}", grade);
            }
        }
        Err(message) => println!("{}", message),
    }

    let student_status = check_and_return_student(&student_name, &student_db);
    match student_status {
        Ok(student_grade) => match student_grade {
            Some(grade) => println!("Grade is {}", grade),
            None => println!("Grade is not evaluated"),
        },
        Err(message) => println!("{}", message),
    }
}
