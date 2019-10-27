pub use pitch_calc::Letter;
pub use pitch_calc::{LetterOctave, Step};

use pitch_calc::letter_octave_from_step;

// TODO: implement Display for interval
pub type Interval = i32;
#[allow(dead_code, non_upper_case_globals)]
pub mod intervals {
    use super::*;
    pub const Root: Interval = 0;
    pub const Min2nd: Interval = 1;
    pub const Maj2nd: Interval = 2;
    pub const Min3rd: Interval = 3;
    pub const Maj3rd: Interval = 4;
    pub const Per4th: Interval = 5;

    pub const Dim5th: Interval = 6;
    pub const Per5th: Interval = 7;
    pub const Aug5th: Interval = 8;

    pub const Min6th: Interval = 8;
    pub const Maj6th: Interval = 9;
    pub const Min7th: Interval = 10;
    pub const Maj7th: Interval = 11;
    pub const Octave: Interval = 12;
}

pub fn transpose(note: LetterOctave, interval: Interval) -> LetterOctave {
    let interval_s = interval as pitch_calc::calc::Step;
    let (letter, octave) = letter_octave_from_step(note.step() + interval_s);
    LetterOctave(letter, octave)
}

// TODO: implement Display for Degree
pub type Degree = i32;
#[allow(dead_code)]
pub mod degrees {
    use super::*;
    pub const I: Degree = 1;
    pub const II: Degree = 2;
    pub const III: Degree = 3;
    pub const IV: Degree = 4;
    pub const V: Degree = 5;
    pub const VI: Degree = 6;
    pub const VII: Degree = 7;
}

pub type Quality = Vec<(Interval, Degree)>;

#[derive(PartialEq, Eq, Debug)]
pub struct Chord {
    pub root: LetterOctave,
    pub quality: Quality,
}

#[allow(dead_code)]
impl Chord {
    pub fn transposed(&self, interval: Interval) -> Chord {
        let transposed = self.root.step() + interval as f32;
        let (letter, octave) = letter_octave_from_step(transposed);
        Chord {
            root: LetterOctave(letter, octave),
            quality: self.quality.clone(),
        }
    }

    pub fn root(&self) -> LetterOctave {
        self.root
    }

    pub fn with_root(&self, new_root: LetterOctave) -> Chord {
        Chord {
            root: new_root,
            quality: self.quality.clone(),
        }
    }

    pub fn root_letter(&self) -> Letter {
        self.root.letter()
    }

    pub fn with_root_letter(&self, new_root: Letter) -> Chord {
        Chord {
            root: LetterOctave(new_root, self.root.octave()),
            quality: self.quality.clone(),
        }
    }

    pub fn quality(&self) -> Quality {
        self.quality.clone()
    }

    pub fn with_quality(&self, quality: Quality) -> Chord {
        Chord {
            root: self.root,
            quality: quality,
        }
    }

    pub fn notes(&self) -> Vec<LetterOctave> {
        self.quality
            .clone()
            .into_iter()
            .map(|(i, _)| transpose(self.root, i))
            .fold(vec![self.root()], |mut ns, n| {
                ns.push(n);
                ns
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use degrees::*;
    use intervals::*;

    #[allow(unused_imports)]
    use pitch_calc::Letter::*;

    #[test]
    fn interval_maths() {
        assert_eq!(C, F + Per5th);
        assert_eq!(C, C + Octave);
        assert_eq!(C, C + Octave + Octave);

        assert_eq!(C, G - Per5th);
        assert_eq!(C, C - Octave);
        assert_eq!(C, C - Per4th - Per5th);
        assert_eq!(C - Min2nd, C + Maj7th);
    }

    #[test]
    #[allow(non_snake_case)]
    fn chord_maths() {
        let Cmaj = Chord {
            root: LetterOctave(Letter::C, 4),
            quality: vec![(Maj3rd, III), (Per5th, V)],
        };

        let Gmaj = Chord {
            root: LetterOctave(Letter::G, 4),
            quality: vec![(Maj3rd, III), (Per5th, V)],
        };

        let Amin = Chord {
            root: LetterOctave(Letter::A, 4),
            quality: vec![(Min3rd, III), (Per5th, V)],
        };

        assert_eq!(Gmaj, Cmaj.transposed(Per5th));
        assert_eq!(Gmaj, Cmaj.with_root(LetterOctave(Letter::G, 4)));
        assert_eq!(Gmaj, Cmaj.with_root_letter(Letter::G));

        let Amin_generated = Cmaj
            .transposed(Maj6th)
            .with_quality(vec![(Min3rd, III), (Per5th, V)]);
        assert_eq!(Amin, Amin_generated);

        let notes_of_c_major: Vec<LetterOctave> = [Letter::C, Letter::E, Letter::G]
            .iter()
            .map(|l| LetterOctave(*l, 4))
            .collect();
        assert_eq!(Cmaj.notes(), notes_of_c_major);
    }
}
