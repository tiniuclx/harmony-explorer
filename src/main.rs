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

use std::error::Error;
use std::sync::{Arc, Mutex};

use rustyline::error::ReadlineError;
use rustyline::Editor;

use gag::Gag;
use sampler::Sampler;

use music_theory::Chord;

const CHANNELS: i32 = 2;
const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 1024;
//const THUMB_PIANO: &'static str = "thumbpiano A#3.wav";
const CASIO_PIANO: &'static str = "Casio Piano C5.wav";

fn main() -> Result<(), Box<dyn Error>> {
    // Initialise audio plumbing and sampler.
    // Suppress warnings from PortAudio
    let gag_stderr = Gag::stderr();

    // We'll create a sample map that maps a single sample to the entire note range.
    let assets = find_folder::Search::ParentsThenKids(5, 5)
        .for_folder("assets")
        .unwrap();
    let sample = sampler::Sample::from_wav_file(assets.join(CASIO_PIANO), SAMPLE_RATE).unwrap();
    let sample_map = sampler::Map::from_single_sample(sample);

    // Create atomic RC pointer to a mutex protecting the polyphonic sampler
    let sampler_arc = Arc::new(Mutex::new(Sampler::poly((), sample_map).num_voices(12)));

    // Initialise PortAudio and create an output stream.
    let pa = pa::PortAudio::new()?;
    let settings =
        pa.default_output_stream_settings::<f32>(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER)?;
    let sampler_arc_callback = sampler_arc.clone();

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
    let db = database::initialise_database().unwrap();
    chord_library::populate_database(&db);

    loop {
        let readline = rl.readline("♪♪♪ ");
        match readline {
            Ok(line) => {
                {
                    use music_theory::degree_intervals::*;
                    use pitch_calc::Letter;
                    use pitch_calc::LetterOctave;

                    let mut sampler = sampler_arc.lock().unwrap();
                    let vel = 0.3;

                    let c_maj = Chord {
                        root: LetterOctave(Letter::C, 4),
                        quality: vec![Maj3rd, Per5th, Maj6th],
                    };

                    c_maj.notes().into_iter().for_each(|n| {
                        sampler.note_on(n.to_hz(), vel);
                    });
                }
                println!("{}", line);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    stream.stop()?;
    stream.close()?;
    Ok(())
}
