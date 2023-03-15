// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    assignment: Option<i32>,
}

impl Student {
    fn print_assignment(&self) {
        match self.assignment {
            Some(locker) => println!("Name: {}, Locker Assignment: {}", self.name, locker),
            None => println!("Name: {}, Locker Assignment: None", self.name),
        }
    }
}

fn main() {
    let pit: Student = Student {
        name: String::from("Pit"),
        assignment: Some(69),
    };
    let greg: Student = Student {
        name: String::from("Greg"),
        assignment: None,
    };
    let maggie: Student = Student {
        name: String::from("Maggie"),
        assignment: Some(20),
    };

    let students: Vec<Student> = vec![pit, greg, maggie];

    for student in students.iter() {
        student.print_assignment();
    }
}
