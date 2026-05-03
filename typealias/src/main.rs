// Setting alias to integer type offering a better understanding of the values.
type Meters = i32;

fn main() {
    let _mile_race_length: Meters = 1600;
    let _two_mile_race_length: Meters = 3200;
    println!("A one mile race is {} meters long and two mile race is {} meters long", _mile_race_length, _two_mile_race_length);
}
