// An array is a fixed-size collection of homogenous data (data of the same type)
fn main() {
    let _numbers: [i32; 6] = [4, 8, 15, 16, 23, 42];

    let _fruit: [&str; 3] = ["Orange", "Pinapple", "Papaia"];
    println!("Lenght: {}", _fruit.len());

    let _currency_rates: [f64; 2] = [1.7, 7.4];
    println!("Lenght: {}", _currency_rates.len());

    // Array Index position
    let mut _seasons: [&str; 4] = ["Spring", "Summer", "Fall", "Winter"];

    let _first: &str = _seasons[0];
    let _second: &str = _seasons[1];
    let _third: &str = _seasons[2];
    let _fourth: &str = _seasons[3];

    println!("The first season is {_first} and the second season is {_second}");
    println!("The first season if {} and the second season is {}", _first, _second);
    println!("The first season if {} and the second season is {}", _seasons[0], _seasons[1]);

    // Replacing the valor of index (using mut feature)
    print!("{}\n", _seasons[2]);
    _seasons[2] = "Autumn";
    print!("{}", _seasons[2]);
}
