// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum Employment {
    Maintenance,
    Marketing,
    Manager,
    Supervisor,
    Kitchen,
    Assembly,
}

enum Status {
    Active,
    Terminated,
}

struct Employee {
    employment: Employment,
    is_employed: Status,
}

impl Employee {
    fn is_still_employed(&self) -> Result<(), String> {
        match self.is_employed {
            Status::Active => {
                get_employment_type(&self)?;
                println!("Access granted!");
                Ok(())
            }
            Status::Terminated => Err("Is not employed".to_owned()),
        }
    }
}

fn get_employment_type(employee: &Employee) -> Result<(), String> {
    match employee.employment {
        Employment::Maintenance | Employment::Marketing | Employment::Manager => Ok(()),
        _ => Err("Not allowed".to_owned()),
    }
}

fn main() {
    let pit: Employee = Employee {
        employment: Employment::Manager,
        is_employed: Status::Active,
    };

    pit.is_still_employed().unwrap();
}
