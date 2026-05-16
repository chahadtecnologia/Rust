fn main() {
    let _seasons: [&str; 4] = ["Spring", "Summer", "Fall", "Winter"];

    println!("{}", _seasons[0]);
    println!("{}", _seasons[1]);
    println!("{}", _seasons[3]);
    // Debug Traits
    println!("{:?}", _seasons);
    println!("{_seasons:?}");
    // A better display of Debug Trait
    println!("{_seasons:#?}");
}
