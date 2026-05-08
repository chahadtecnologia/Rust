// Casting means to transform a type in another (convergion)
fn main() {
    let _miles_away = 50;
    let _miles_away_i8 = _miles_away as i8;
    let _miles_away_u8 = _miles_away as u8;

    let _miles_away = 100.329032;
    let _miles_away_f32 = _miles_away as f32;
    let _miles_away_int = _miles_away as i32;
    println!("{_miles_away_int}");
}
