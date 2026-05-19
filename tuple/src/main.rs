/* 
The difference between Tuple and Array is the Tuple support multiple data types.
In Array the data types must be the same.
*/
fn main() {
    let _employee: (&str, i32, &str) = ("MaGus", 45, "CyberSec");

    let _name: &str = _employee.0;
    let _age: i32 = _employee.1;
    let _department: &str = _employee.2;

    // Same result above but in a single line
    let (_name, _age, _department) = _employee;

    println!("Name: {_name}, age: {_age}, department: {_department}");

    print!("{_employee:#?}");
}
