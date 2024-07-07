#[derive(Debug)]
struct Employee {
    name: String,
    salary: u16,
}

struct EmployeeRecords {
    employee_db: Vec<Employee>,
}

impl Iterator for EmployeeRecords {
    type Item = Employee;

    fn next(&mut self) -> Option<Self::Item> {
        if self.employee_db.len() != 0 {
            let result = self.employee_db.remove(0);
            Some(result)
        } else {
            None
        }
    }
}

fn main() {
    let emp_1 = Employee {
        name: String::from("a"),
        salary: 1,
    };
    let emp_2 = Employee {
        name: String::from("b"),
        salary: 1,
    };
    let mut emp_db = EmployeeRecords {
        employee_db: vec![emp_1, emp_2]
    };

    for employee in emp_db {
        println!("{:?}", employee)
    }
}
