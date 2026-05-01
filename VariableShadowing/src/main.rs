fn main() {
    
    // Variable Shadowing is the reuse of same variable, defining new value with different types.
    let _grams_of_protein: &str = "100.345";
    println!("Grams of protein using string type: {}", _grams_of_protein);

    /* 
        Block of Rust code using the string type.
    */

    let _grams_of_protein: f64 = 100.345;
    println!("Grams of protein using floating type: {}", _grams_of_protein);

    /*
        Block of Rust code using the floating type.
    */

    let _grams_of_protein: i32 = 100;
    println!("Grams of protein using integer type: {}", _grams_of_protein);

    /*
        Block of Rust code using integer type.
    */
}
