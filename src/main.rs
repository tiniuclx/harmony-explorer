extern crate enum_primitive_derive;
extern crate num_traits;

mod music_theory;
use pitch_calc::Letter::*;

fn main() {
    println!("{:?}", C);
    println!("{:?}", C + 7);
    println!("{:?}", C + 12);
    println!("{:?}", C + 48);
    println!("{:?}", C - 2);
    println!("{:?}", C - 7);
    println!("{:?}", C - 22);
}
