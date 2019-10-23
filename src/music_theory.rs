extern crate num_traits;

use num_traits::{FromPrimitive, ToPrimitive};
use std::ops;

#[derive(Debug, PartialEq, Eq, Primitive)]
pub enum Note {
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
impl ops::Add<i64> for Note {
    type Output = Note;

    fn add(self, other: i64) -> Note {
        let note = Note::to_i64(&self).expect("Note::to_i64 failed!");
        let transposed = (note + other).rem_euclid(12);

        Note::from_i64(transposed).expect("Note::from_i64 failed!")
    }
}

impl ops::Sub<i64> for Note {
    type Output = Note;

    fn sub(self, other: i64) -> Note {
        self + (-other)
    }
}

#[allow(dead_code)]
pub enum Interval {
    Unison,
    Min2nd,
    Maj2nd,
    Min3rd,
    Maj3rd,
    Per4th,
    Dim5th,
    Per5th,
    Min6th,
    Maj6th,
    Min7th,
    Maj7th,
    Octave,
}

impl ops::Add<Interval> for Note {
    type Output = Note;

    fn add(self, other: Interval) -> Note {
        self + other as i64
    }
}

impl ops::Sub<Interval> for Note {
    type Output = Note;

    fn sub(self, other: Interval) -> Note {
        self - other as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Interval::*;

    #[test]
    fn small_transposition() {
        assert_eq!(Note::G, Note::C + 7);
    }

    #[test]
    fn big_transposition() {
        assert_eq!(Note::C, Note::C + 12);
        assert_eq!(Note::C, Note::C + 24);
        assert_eq!(Note::C, Note::C + 144);
    }

    #[test]
    fn transpose_down() {
        assert_eq!(Note::Db, Note::D - 1);
        assert_eq!(Note::C, Note::G - 7);
        assert_eq!(Note::C, Note::C - 24);
    }

    #[test]
    fn interval_maths() {
        assert_eq!(Note::C, Note::F + Per5th);
        assert_eq!(Note::C, Note::C + Octave);
        assert_eq!(Note::C, Note::C + Octave + Octave);

        assert_eq!(Note::C, Note::G - Per5th);
        assert_eq!(Note::C, Note::C - Octave);
        assert_eq!(Note::C, Note::C - Per4th - Per5th);
        assert_eq!(Note::C - Min2nd, Note::C + Maj7th);
    }
}
