fn main() {
    let _seasons: [&str; 4] = ["Spring", "Summer", "Autumn", "Winter"];

    println!("{}", _seasons[0]);
    println!("{}", _seasons[1]);
    println!("{}", _seasons[2]);
    println!("{}", _seasons[3]);

    // Debug Macro is a helper to understand more about the Rust code
    dbg!(_seasons[0]);
    dbg!(_seasons);
}
