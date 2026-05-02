fn main() {
    let _coffee_price: f64 = 5.99;

    // Here we can access the default name of variable.
    println!("The coffee price is {}", _coffee_price);

    {
        // Setting a new value for _coffee_price is creating a separate and independent variable
        let _coffee_price: f64 = 4.99;
        println!("The coffee price is {}", _coffee_price);

        // Creating a new variable, _cookie_price.
        let _cookie_price: f64 = 1.99;
        println!("The cookie price is {}", _cookie_price);
    }

    // Here, in this scope, we have the default value of _coffee_price.
    println!("The coffee price is {}", _coffee_price);

    // Here is not possible to access the _cookie_price variable. Out of scope.
    println!("The cookie price is {}", _cookie_price);
}
