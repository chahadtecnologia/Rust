// A format specifier customizes the printed representation of the interpolated value.
fn main() {
    let pi: f64 = 3.1415926535897932384;
    println!("The current value of pi is {pi:.2}"); // 2 digits of precision
    println!("The current value of pi is {:.2}", pi); // 2 digits of precision
    println!("The current value of pi is {pi:.4}"); // 4 digits of precision
    println!("The current value of pi is {:.4}", pi); // 4 digits of precision
    println!("The current value of pi is {pi:.8}"); // 8 digits of precision
    println!("The current value of pi is {:.8}", pi); // 8 digits of precision
}
