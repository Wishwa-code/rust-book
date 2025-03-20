use std::collections::HashMap;
use std::io;

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Enter a command (e.g., 'Add Sally to Engineering', 'List Engineering', 'List All', or 'Quit'):");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim(); // Remove leading/trailing whitespace

        match input.split_whitespace().collect::<Vec<&str>>().as_slice() {
            ["Add", name, "to", department] => {
                add_employee(&mut company, name.to_string(), department.to_string());
                println!("Added {} to {}", name, department);
            }
            ["List", department] => {
                list_department(&company, department);
            }
            ["ListAll"] => {
                list_all_departments(&company);
            }
            ["Quit"] => {
                println!("Exiting program.");
                break;
            }
            _ => {
                println!("Invalid command. Please try again.");
            }
        }
    }
}

fn add_employee(company: &mut HashMap<String, Vec<String>>, name: String, department: String) {
    company
        .entry(department)
        .or_insert_with(Vec::new)
        .push(name);
}

fn list_department(company: &HashMap<String, Vec<String>>, department: &str) {
    match company.get(department) {
        Some(employees) => {
            let mut sorted_employees = employees.clone();
            sorted_employees.sort();
            println!("Employees in {}: {:?}", department, sorted_employees);
        }
        None => {
            println!("Department '{}' does not exist.", department);
        }
    }
}

fn list_all_departments(company: &HashMap<String, Vec<String>>) {
    if company.is_empty() {
        println!("No departments found.");
        return;
    }

    let mut departments: Vec<&String> = company.keys().collect();
    departments.sort();

    for department in departments {
        list_department(company, department);
    }
}