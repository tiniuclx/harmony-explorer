#[macro_use]
extern crate enum_primitive_derive;

mod music_theory;
use music_theory::MusicalNote;

fn main() {
    println!("{:?}", MusicalNote::C);
    println!("{:?}", MusicalNote::C + 7);
    println!("{:?}", MusicalNote::C + 12);
    println!("{:?}", MusicalNote::C + 48);
    println!("{:?}", MusicalNote::C - 2);
    println!("{:?}", MusicalNote::C - 7);
    println!("{:?}", MusicalNote::C - 22);
}
