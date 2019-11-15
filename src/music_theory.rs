pub use pitch_calc::{Letter, LetterOctave, Step};
use std::fmt;

use pitch_calc::letter_octave_from_step;

// TODO: implement Display for Degree
pub type Degree = i32;
#[allow(dead_code)]
/// Helper module exporting constants for roman numeral chord and scale degrees.
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

// TODO: implement Display for interval
pub type Interval = i32;
#[allow(dead_code, non_upper_case_globals)]
/// Helper module containing intervals, and their size in semitones.
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
    pub const Dim7th: Interval = 9;
    pub const Min7th: Interval = 10;
    pub const Maj7th: Interval = 11;

    pub const Octave: Interval = 12;
}

#[allow(dead_code, non_upper_case_globals)]
// Contains constants of the type (Degree, Interval). Useful mostly for building Chords.
pub mod degree_intervals {
    use super::*;
    use degrees::*;
    pub const Root: (Degree, Interval) = (I, intervals::Root);

    pub const Min2nd: (Degree, Interval) = (II, intervals::Min2nd);
    pub const Maj2nd: (Degree, Interval) = (II, intervals::Maj2nd);

    pub const Min3rd: (Degree, Interval) = (III, intervals::Min3rd);
    pub const Maj3rd: (Degree, Interval) = (III, intervals::Maj3rd);

    pub const Per4th: (Degree, Interval) = (IV, intervals::Per4th);

    pub const Dim5th: (Degree, Interval) = (V, intervals::Dim5th);
    pub const Per5th: (Degree, Interval) = (V, intervals::Per5th);
    pub const Aug5th: (Degree, Interval) = (V, intervals::Aug5th);

    pub const Min6th: (Degree, Interval) = (VI, intervals::Min6th);
    pub const Maj6th: (Degree, Interval) = (VI, intervals::Maj6th);

    pub const Dim7th: (Degree, Interval) = (VII, intervals::Dim7th);
    pub const Min7th: (Degree, Interval) = (VII, intervals::Min7th);
    pub const Maj7th: (Degree, Interval) = (VII, intervals::Maj7th);

    pub const Octave: (Degree, Interval) = (I, intervals::Octave);
}

pub type Quality = Vec<(Degree, Interval)>;

/// Chords are composed of the root tone, followed by a list of notes
/// and their scale degrees.
#[derive(PartialEq, Eq, Debug)]
pub struct Chord {
    pub root: LetterOctave,
    pub quality: Quality,
}

/// Transpose the note by the number of semitones in `interval`.
pub fn transpose(note: LetterOctave, interval: Interval) -> LetterOctave {
    let interval_s = interval as pitch_calc::calc::Step;
    let (letter, octave) = letter_octave_from_step(note.step() + interval_s);
    LetterOctave(letter, octave)
}

#[allow(dead_code)]
impl Chord {
    /// Returns a new chord, transposed by `interval`.
    pub fn transposed(&self, interval: Interval) -> Chord {
        Chord {
            root: transpose(self.root, interval),
            quality: self.quality.clone(),
        }
    }

    /// Returns the root note of the chord.
    pub fn root(&self) -> LetterOctave {
        self.root
    }

    /// Returns a new chord with the same quality but with a different root,
    /// determined by `new_root`.
    pub fn with_root(&self, new_root: LetterOctave) -> Chord {
        Chord {
            root: new_root,
            quality: self.quality.clone(),
        }
    }

    /// Returns the `Letter` of the root.
    pub fn root_letter(&self) -> Letter {
        self.root.letter()
    }

    /// Returns a new chord with the same quality but with a different root,
    /// determined by `new_root`. The new chord will be voiced in the same
    /// octave as the old one.
    pub fn with_root_letter(&self, new_root: Letter) -> Chord {
        Chord {
            root: LetterOctave(new_root, self.root.octave()),
            quality: self.quality.clone(),
        }
    }

    /// Returns a copy of the chord's Quality.
    pub fn quality(&self) -> Quality {
        self.quality.clone()
    }

    /// Returns a new chord with the same root, but a different `quality`.
    pub fn with_quality(&self, quality: Quality) -> Chord {
        Chord {
            root: self.root,
            quality: quality,
        }
    }

    /// Returns all of the notes that make up the chord.
    pub fn notes(&self) -> Vec<LetterOctave> {
        self.quality
            .clone()
            .into_iter()
            .map(|(_, i)| transpose(self.root, i))
            .fold(vec![self.root()], |mut ns, n| {
                ns.push(n);
                ns
            })
    }
}

static mut use_flats: bool = false;

pub fn set_use_flats(flats: bool) {
    unsafe {
        use_flats = flats;
    }
}

pub fn get_use_flats() -> bool {
    use_flats
}

pub fn letter_to_string(letter: Letter) -> String {
    use pitch_calc::Letter::*;
    let flats = get_use_flats();
    let s = match letter {
        C => "C",
        D => "D",
        E => "E",
        F => "F",
        G => "G",
        A => "A",
        B => "B",

        Db | Csh => {
            if flats {
                "Db"
            } else {
                "C#"
            }
        }

        Eb | Dsh => {
            if flats {
                "Eb"
            } else {
                "D#"
            }
        }

        Gb | Fsh => {
            if flats {
                "Gb"
            } else {
                "F#"
            }
        }

        Ab | Gsh => {
            if flats {
                "Ab"
            } else {
                "G#"
            }
        }

        Bb | Ash => {
            if flats {
                "Bb"
            } else {
                "A#"
            }
        }
    };
    s.to_string()
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use pitch_calc::Letter::*;

    #[test]
    fn interval_maths() {
        use intervals::*;
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
        use degree_intervals::*;
        let Cmaj = Chord {
            root: LetterOctave(Letter::C, 4),
            quality: vec![Maj3rd, Per5th],
        };

        let Gmaj = Chord {
            root: LetterOctave(Letter::G, 4),
            quality: vec![Maj3rd, Per5th],
        };

        let Amin = Chord {
            root: LetterOctave(Letter::A, 4),
            quality: vec![Min3rd, Per5th],
        };

        assert_eq!(Gmaj, Cmaj.transposed(intervals::Per5th));
        assert_eq!(Gmaj, Cmaj.with_root(LetterOctave(Letter::G, 4)));
        assert_eq!(Gmaj, Cmaj.with_root_letter(Letter::G));

        let Amin_generated = Cmaj
            .transposed(intervals::Maj6th)
            .with_quality(vec![Min3rd, Per5th]);
        assert_eq!(Amin, Amin_generated);

        let notes_of_c_major: Vec<LetterOctave> = [Letter::C, Letter::E, Letter::G]
            .iter()
            .map(|l| LetterOctave(*l, 4))
            .collect();
        assert_eq!(Cmaj.notes(), notes_of_c_major);
    }
}
