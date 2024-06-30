use std::collections::HashMap;

#[derive(Debug)]
struct StudentQ1 {
    id: i32,
    name: String,
    grade: String,
}

struct StudentManager {
    students: HashMap<i32, StudentQ1>,
}

impl StudentManager {
    fn new() -> Self {
        Self {
            students: HashMap::new(),
        }
    }

    fn add_student(&mut self, student: StudentQ1) -> Result<(), String> {
        if self.students.contains_key(&student.id) {
            return Err(String::from("student already exists"));
        }
        self.students.insert(student.id, student);
        Ok(())
    }

    fn get_student(&self, id: i32) -> Option<&StudentQ1> {
        self.students.get(&id)
    }
}

pub fn q1() {
    let mut student_manager = StudentManager::new();
    let student1 = StudentQ1 {
        id: 1,
        name: String::from("Alice"),
        grade: String::from("A"),
    };
    let student2 = StudentQ1 {
        id: 2,
        name: String::from("Bob"),
        grade: String::from("B"),
    };
    student_manager.add_student(student1).unwrap();
    student_manager.add_student(student2).unwrap();
    let student = student_manager.get_student(1);
    println!("{:?}", student);
}

struct StudentQ2 {
    name: String,
    age: i32,
    grade: String,
}

fn add_student(
    student_database: &mut HashMap<i32, StudentQ2>,
    id: i32,
    name: String,
    age: i32,
    grade: String,
) {
    if !student_database.contains_key(&id) {
        let new_student = StudentQ2 {
            name: name,
            age: age,
            grade: grade,
        };
        student_database.insert(id, new_student);
    }
}

pub fn q2() {
    let mut student_database: HashMap<i32, StudentQ2> = HashMap::new();
    add_student(
        &mut student_database,
        1,
        String::from("John"),
        17,
        String::from("Grade 11"),
    );

    add_student(
        &mut student_database,
        2,
        String::from("Sarah"),
        16,
        String::from("Grade 10"),
    );
    
    for (id, student) in &student_database {
        println!("Student ID: {}", id);
        println!("Name: {}", student.name);
        println!("Age: {}", student.age);
        println!("Grade: {}", student.grade);
        println!("------------------");
    }
}
