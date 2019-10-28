use crate::music_theory::*;
use nom::character::complete::{multispace0, not_line_ending};
use nom::*;
use std::collections::HashMap;

/// This is the abstract syntax tree of the REPL. It describes the syntax of
/// every command that can be used.
pub enum Command {
    /// Nothing at all was typed.
    EmptyString,
    /// A valid note letter followed by the chord quality.
    Chord(Letter, String),
}

named! { letter_accidental (&str) -> String,
    do_parse!(
        letter: one_of!("ABCDEFG") >>
        accidental: complete!(alt!(char!('#') | char!('b'))) >>
        ( [letter, accidental].into_iter().collect() )
    )
}

named! { letter_natural (&str) -> String,
    do_parse!(
        letter: one_of!("ABCDEFG") >>
        ( letter.to_string() )
    )
}

named! { select_letter (&str) -> String,
    alt!(letter_accidental | letter_natural)
}

fn note_map() -> HashMap<String, Letter> {
    use Letter::*;
    [
        ("C", C),
        ("C#", Csh),
        ("Db", Db),
        ("D", D),
        ("D#", Dsh),
        ("Eb", Eb),
        ("E", E),
        ("F", F),
        ("F#", Fsh),
        ("Gb", Gb),
        ("G", G),
        ("G#", Gsh),
        ("Ab", Ab),
        ("A", A),
        ("A#", Ash),
        ("Bb", Bb),
        ("B", B),
    ]
    .into_iter()
    .map(|t| ((t.0).to_string(), t.1))
    .collect()
}

named! { pub letter (&str) -> Letter,
    map_opt!(select_letter, |s: String| note_map().get(&s).map(|l| l.clone()))
}

named! { command_chord (&str) -> Command,
    do_parse!(
        letter: letter >>
        chord: not_line_ending >>
        (Command::Chord(letter, chord.trim().to_string()))
    )
}

named! { command_null (&str) -> Command,
    map!(multispace0, |_| Command::EmptyString)
}

named! { pub parse_command (&str) -> Command,
    alt!(
        command_chord |
        command_null
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::Err::*;
    use nom::Needed::*;
    use Letter::*;

    #[test]
    fn letter_naturals() {
        assert_eq!(letter("A"), Ok(("", A)));
        assert_eq!(letter("D"), Ok(("", D)));
        assert_eq!(letter("E"), Ok(("", E)));
        assert_eq!(letter("E7b5"), Ok(("7b5", E)));

        assert_eq!(letter(""), Err(Incomplete(Size(1))));
        assert_ne!(letter("a"), Ok(("", A)));
    }

    #[test]
    fn letter_accidentals() {
        assert_eq!(letter("Db"), Ok(("", Db)));
        assert_eq!(letter("Eb"), Ok(("", Eb)));
        assert_eq!(letter("G#"), Ok(("", Gsh)));
        assert_eq!(letter("Bb7add9"), Ok(("7add9", Bb)));

        assert_eq!(letter(""), Err(Incomplete(Size(1))));
        assert_ne!(letter("Db"), Ok(("", D)));
        assert_ne!(letter("E#"), Ok(("", F)));
    }
}
