// Unicode is a computing standard for the representation of text for most of the world's writing systems.
// Character it is a value that supports behaviors or operations through the use of methods.
fn main() {
    let _first_initial = 'B';
    let _emoji = '🎵';

    println!("{} {}", _first_initial.is_alphabetic(),
                        _emoji.is_alphabetic()
    );

    println!("{} {}", _first_initial.is_uppercase(),
                        _emoji.is_uppercase()
    );
}
