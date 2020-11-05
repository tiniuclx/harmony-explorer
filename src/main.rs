#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate nom;

extern crate enum_primitive_derive;
extern crate find_folder; // For easily finding the assets folder.
extern crate num_traits;
extern crate pitch_calc as pitch; // To work with musical notes.
extern crate portaudio as pa; // For audio I/O
extern crate rustyline;
extern crate sample; // To convert portaudio sample buffers to frames.
extern crate sampler;

mod chord_library;
mod database;
mod music_theory;
mod parser;
mod schema;
mod sequencer;

use diesel::SqliteConnection;
use std::error::Error;
use std::sync::{Arc, Mutex};

use rustyline::error::ReadlineError;
use rustyline::Editor;

use sampler::Sampler;

use parser::{parse_command, Command};

const CHANNELS: i32 = 2;
const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 1024;
//const THUMB_PIANO: &'static str = "thumbpiano A#3.wav";
const CASIO_PIANO: &'static str = "Casio Piano C5.wav";

type ArcSampler = Arc<
    Mutex<
        sampler::Sampler<
            sampler::instrument::mode::Poly,
            (),
            Arc<sampler::audio::wav::Audio<[f32; 2]>>,
        >,
    >,
>;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialise audio plumbing and sampler.

    // We'll create a sample map that maps a single sample to the entire note range.
    let assets = find_folder::Search::ParentsThenKids(5, 5).for_folder("assets")?;
    let sample = sampler::Sample::from_wav_file(assets.join(CASIO_PIANO), SAMPLE_RATE)?;
    let sample_map = sampler::Map::from_single_sample(sample);

    // Create atomic RC pointer to a mutex protecting the polyphonic sampler
    let arc_sampler: ArcSampler =
        Arc::new(Mutex::new(Sampler::poly((), sample_map).num_voices(12)));

    // Initialise PortAudio and create an output stream.
    let pa = pa::PortAudio::new()?;
    let settings =
        pa.default_output_stream_settings::<f32>(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER)?;
    let sampler_arc_callback = arc_sampler.clone();

    // Callback is frequently called by PortAudio to fill the audio buffer with samples,
    // which generates sound. Do not do expensive or blocking things in this function!
    let callback = move |pa::OutputStreamCallbackArgs { buffer, .. }| {
        let buffer: &mut [[f32; CHANNELS as usize]] =
            sample::slice::to_frame_slice_mut(buffer).unwrap();
        sample::slice::equilibrium(buffer);

        let mut sampler = sampler_arc_callback.lock().unwrap();
        sampler.fill_slice(buffer, SAMPLE_RATE);

        pa::Continue
    };

    let mut stream = pa.open_non_blocking_stream(settings, callback)?;
    stream.start()?;

    // Audio initialisation is complete. Start processing keyboard input.
    let mut rl = Editor::<()>::new();
    if let Err(_) = rl.load_history(".music_repl_history") {
        // No previous history - that's okay!
    }

    // In-memory SQLite database of chords
    let db = database::initialise_database().unwrap();
    chord_library::populate_database(&db);

    // The last non-empty command is stored here, to be executed again
    // based on user input.
    let mut last_command: Option<Command> = None;

    loop {
        let readline = rl.readline("♪♪♪ ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                match parse_command(&line) {
                    Ok(("", command)) => {
                        // Act based on the received command, and save it if it
                        // is not empty.
                        execute(&command, &last_command, &arc_sampler, &db);
                        if command != Command::EmptyString {
                            last_command = Some(command);
                        }
                    }
                    Ok((remaining, _)) => {
                        // Should not get here, the parser should consume all input
                        println!("Could not process input: {}", remaining);
                    }
                    Err(e) => println!("Error encountered while parsing command: {:?}", e),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C, exiting...");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D, exiting...");
                break;
            }
            Err(err) => {
                println!("Input Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history(".music_repl_history").unwrap();
    stream.close()?;
    Ok(())
}

// Ideally this function should be as small as possible -
// all the work should be done in the functional core,
// the command parser. All this function must do is
// glue the different modules together
fn execute(
    command: &Command,
    last_command: &Option<Command>,
    arc_sampler: &ArcSampler,
    db: &SqliteConnection,
) {
    use music_theory::*;
    match command {
        // Look up the chord quality in the database, play it and
        // print its notes.
        Command::Chord(letter, quality) => {
            use database::*;
            let mut sampler = arc_sampler.lock().unwrap();
            let vel = 0.6;
            match get_quality(&quality, &db) {
                Some(q) => {
                    let retrieved_quality = q;
                    let chord = Chord {
                        root: LetterOctave(*letter, 4),
                        quality: retrieved_quality,
                    };
                    chord.notes().into_iter().for_each(|n| {
                        sampler.note_on(n.to_hz(), vel);
                    });

                    println!("Playing {}", chord);
                }
                None => {
                    println!("Could not find chord!");
                }
            }
        }
        // Re-do the last command.
        Command::EmptyString => match last_command {
            Some(Command::EmptyString) => (),
            Some(c) => execute(&c, &None, arc_sampler, db),
            None => (),
        },

        Command::Flats => {
            set_use_flats(true);
            println!("Notating accidentals using flats.");
        }

        Command::Sharps => {
            set_use_flats(false);
            println!("Notating accidentals using sharps.");
        }

        Command::Transpose(distance, letter, quality) => {
            let new_letter = *letter + *distance;
            let new_command = Command::Chord(new_letter, quality.to_string());

            println!("{}{}", letter_to_string(new_letter), quality.to_string());
            execute(&new_command, last_command, arc_sampler, db);
        }
    };
}
