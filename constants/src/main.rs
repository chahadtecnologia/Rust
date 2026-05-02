/*
    A constant is a name assigned to a value. A constant's value cannot change.
    A constant could be used in any scope.
    A constant value could be define when the program is running.
*/

const TAX_RATE: f64 = 7.25;

fn main() {
    let _income: i32 = 100000;
    println!("My income is {} and the tax rate is {}", _income, TAX_RATE);
}
