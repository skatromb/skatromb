use std::collections::HashMap;
/// Using a hash map and vectors, create a text interface
/// to allow a user to add employee names to a department in a company.
/// For example, “Add Sally to Engineering” or “Add Amir to Sales.”
/// Then let the user retrieve a list of all people in a department
/// or all people in the company by department, sorted alphabetically.
use std::io;

fn add_user(deps_emps: &mut HashMap<String, Vec<String>>) {
    println!("Enter department to add an employee:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let department = input.trim().to_string();

    println!("Enter employee name to add:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let employee = input.trim().to_string();

    deps_emps.entry(department).or_default().push(employee)
}

fn list_department(deps_emps: &HashMap<String, Vec<String>>) {
    println!("Which department to list employees from? Leave blank to show all:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    match input.trim() {
        "" => println!(
            "Here is the list of whole company employees: {:?}",
            deps_emps
        ),
        dep => {
            println!(
                "Here is the list of employees for {} department: {:?}",
                dep, deps_emps
            )
        }
    }
}

fn main() {
    let mut deps_emps: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!("Enter \"add\" if you want to add employee to department.");
        println!("Enter \"list\" if you want to list the users.");
        println!("Enter \"exit\" if you want end the program.");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().to_lowercase().as_str() {
            "add" => add_user(&mut deps_emps),
            "list" => list_department(&deps_emps),
            "exit" => break,
            _ => {
                println!("Improper variant `{}`", input);
                continue;
            }
        }
    }
}
