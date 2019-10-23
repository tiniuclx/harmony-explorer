#[macro_use]
extern crate enum_primitive_derive;

mod music_theory;
use music_theory::Note;

fn main() {
    println!("{:?}", Note::C);
    println!("{:?}", Note::C + 7);
    println!("{:?}", Note::C + 12);
    println!("{:?}", Note::C + 48);
    println!("{:?}", Note::C - 2);
    println!("{:?}", Note::C - 7);
    println!("{:?}", Note::C - 22);
}
