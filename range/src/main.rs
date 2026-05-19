use std::ops::Range;
use std::ops::RangeInclusive;
// A range is a sequence/interval of consecutive values
fn main() {
    // This range is 1 to 30, not including 31 in the result
    let _month_days: Range<i32> = 1..31;
    println!("{_month_days:?}");

    // This range is 1 to 31, including 31 in the result
    let _month_days: RangeInclusive<i32> = 1..=31;
    println!("{_month_days:?}");

    // To iterate means to progress over the elements of a collection one by one.
    for number in _month_days {
        println!("{number}");
    }

    // This range is a to e, not including f in the result
    let _letters: Range<char> = 'a'..'f';

    for letter in _letters {
        println!("{letter}");
    }

    let _colors: [&str; 3] = ["Red", "Green", "Yellow"];

    for color in _colors {
        println!("{color} if a great color!");
    }

}
