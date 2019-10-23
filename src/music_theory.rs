extern crate num_traits;

use num_traits::{FromPrimitive, ToPrimitive};
use std::ops;

#[derive(Debug, Primitive)]
pub enum MusicalNote {
    A = 0,
    Bb = 1,
    B = 2,
    C = 3,
    Db = 4,
    D = 5,
    Eb = 6,
    E = 7,
    F = 8,
    Gb = 9,
    G = 10,
    Ab = 11,
}
impl ops::Add<i64> for MusicalNote {
    type Output = MusicalNote;

    fn add(self, other: i64) -> MusicalNote {
        let note = MusicalNote::to_i64(&self).expect("MusicalNote::to_i64 failed!");
        let transposed = (note + other).rem_euclid(12);

        MusicalNote::from_i64(transposed).expect("MusicalNote::from_i64 failed!")
    }
}

impl ops::Sub<i64> for MusicalNote {
    type Output = MusicalNote;

    fn sub(self, other: i64) -> MusicalNote {
        self + (-other)
    }
}
