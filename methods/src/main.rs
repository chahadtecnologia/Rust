// A method is a function that lives on a value. It's an action we can ask the value to execute.
// value.method()

fn main() {
    let value: i32 = -15;
    println!("{}", value.abs());

    let empty_space: &str = "   my content  ";
    println!("{}", empty_space.trim());

    println!("{}", value.pow(2));
    println!("{}", value.pow(3));
}
