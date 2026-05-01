fn main() {
    let _apples: i32 = 50;
    let _oranges: i32 = 14 + 6;
    let _fruits: i32 = _apples + _oranges;

    // We have two ways to use variables in println output
    println!("This year, my garden has {} apples.", _apples);
    println!("This year, my garden has {_apples} apples.");

    println!("This year, my garden has {} oranges.", _oranges);
    println!("This year, my garden has {_oranges} oranges.");

    println!("This year, my garden has {} apples and {} oranges.", _apples, _oranges);
    println!("This year, my garden has {_apples} apples and {_oranges} oranges.");

    println!("This year, my garden has {} fruits.", _fruits);
    println!("This year, my garden has {_fruits} fruits.");

    // Positional Arguments to println
    println!("This year, my garden has {0} apples and {1} oranges. The {0} apples are red.", _apples, _oranges);
}
