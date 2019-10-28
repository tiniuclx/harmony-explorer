#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate nom;

extern crate enum_primitive_derive;
extern crate find_folder; // For easily finding the assets folder.
extern crate gag;
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

use diesel::SqliteConnection;
use std::error::Error;
use std::sync::{Arc, Mutex};

use rustyline::error::ReadlineError;
use rustyline::Editor;

use gag::Gag;
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
    // Suppress warnings from PortAudio
    // TODO: route these to an error file for debugging
    let gag_stderr = Gag::stderr();

    // We'll create a sample map that maps a single sample to the entire note range.
    let assets = find_folder::Search::ParentsThenKids(5, 5)
        .for_folder("assets")
        .unwrap();
    let sample = sampler::Sample::from_wav_file(assets.join(CASIO_PIANO), SAMPLE_RATE).unwrap();
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
        let gag_stderr = Gag::stderr();
        let buffer: &mut [[f32; CHANNELS as usize]] =
            sample::slice::to_frame_slice_mut(buffer).unwrap();
        sample::slice::equilibrium(buffer);

        let mut sampler = sampler_arc_callback.lock().unwrap();
        sampler.fill_slice(buffer, SAMPLE_RATE);

        drop(gag_stderr);
        pa::Continue
    };

    let mut stream = pa.open_non_blocking_stream(settings, callback)?;
    stream.start()?;
    drop(gag_stderr);

    // Audio initialisation is complete. Start processing keyboard input.
    let mut rl = Editor::<()>::new();
    if rl.load_history(".music_repl_history").is_err() {
        // No previous history - that's okay!
    }
    let db = database::initialise_database().unwrap();
    chord_library::populate_database(&db);

    loop {
        let readline = rl.readline("♪♪♪ ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                match parse_command(&line) {
                    Ok(("", command)) => execute(command, &arc_sampler, &db),
                    Ok((remaining, command)) => {
                        execute(command, &arc_sampler, &db);
                        println!("Warning: could not process input: {}", remaining);
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
    stream.stop()?;
    stream.close()?;
    Ok(())
}

// Ideally this function should be as small as possible -
// all the work should be done in the functional core,
// the command parser. All this function must do is
// glue the different modules together
fn execute(command: Command, arc_sampler: &ArcSampler, db: &SqliteConnection) {
    match command {
        Command::Chord(letter, quality) => {
            use database::*;
            use music_theory::*;
            let mut sampler = arc_sampler.lock().unwrap();
            let vel = 0.3;
            match get_quality(&quality, &db) {
                Some(q) => {
                    let retrieved_quality = q;
                    let chord = Chord {
                        root: LetterOctave(letter, 4),
                        quality: retrieved_quality,
                    };
                    chord.notes().into_iter().for_each(|n| {
                        sampler.note_on(n.to_hz(), vel);
                    });

                    println!("Playing {:?}", chord);
                }
                None => {
                    println!("Could not find chord!");
                }
            }
        }
        Command::EmptyString => println!("TODO: redo last working command, or print newline"),
    };
}
