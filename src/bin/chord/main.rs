extern crate clap;

use clap::Clap;
use harmony_explorer as hexp;

use hexp::music_theory::{Chord, LetterOctave};

/// Print the notes of a given chord.
#[derive(Clap)]
#[clap(name = "Chord: Print the notes of a given chord")]
#[clap(author = "Alexandru Tiniuc <tiniuc.alexandru@gmail.com>")]
struct Opts {
    /// Name of the chord to search for.
    chord: String,
    /// Octave to append to notes. By default, notes are output without octave number.
    #[clap(short, long, default_value = "3")]
    // TODO: make this optional & print notes without octave
    octave: i32,
}

// TODO: add options for flats/sharps, inversions...
// TODO: add reading from STDIN

fn main() {
    // Initialise chord database
    use hexp::database::*;
    let db = initialise_database().unwrap();
    hexp::chord_library::populate_database(&db);

    let opts = Opts::parse();

    // Parse chord within `opts.chord` CLI field
    match hexp::parser::command_chord(&opts.chord) {
        Ok(("", hexp::parser::Command::Chord(letter, quality))) => {
            // Look up parsed chord within database
            match get_quality(&quality, &db) {
                Some(q) => {
                    // If found, output to stdout!
                    let chord = Chord {
                        root: LetterOctave(letter, opts.octave),
                        quality: q,
                    };
                    println!("{}", chord);
                }
                None => {
                    eprintln!("Could not find chord {}!", opts.chord);
                }
            };
        }
        _ => {
            eprintln!("Invalid input!");
        }
    };
}
