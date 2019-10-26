extern crate enum_primitive_derive;
extern crate find_folder; // For easily finding the assets folder.
extern crate num_traits;
extern crate pitch_calc as pitch; // To work with musical notes.
extern crate portaudio as pa; // For audio I/O
extern crate rustyline;
extern crate sample; // To convert portaudio sample buffers to frames.
extern crate sampler;

mod music_theory;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use sampler::Sampler;
use std::error::Error;

const CHANNELS: i32 = 2;
const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 64;
//const THUMB_PIANO: &'static str = "thumbpiano A#3.wav";
const CASIO_PIANO: &'static str = "Casio Piano C5.wav";

fn main() -> Result<(), Box<dyn Error>> {
    let mut rl = Editor::<()>::new();

    // We'll create a sample map that maps a single sample to the entire note range.
    let assets = find_folder::Search::ParentsThenKids(5, 5)
        .for_folder("assets")
        .unwrap();
    let sample = sampler::Sample::from_wav_file(assets.join(CASIO_PIANO), SAMPLE_RATE).unwrap();
    let sample_map = sampler::Map::from_single_sample(sample);

    // Create a polyphonic sampler.
    let mut sampler = Sampler::poly((), sample_map).num_voices(12);

    // Initialise PortAudio and create an output stream.
    let pa = pa::PortAudio::new()?;
    println!("Device count: {:?}", pa.device_count());
    let settings =
        pa.default_output_stream_settings::<f32>(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER)?;

    let callback = move |pa::OutputStreamCallbackArgs { buffer, .. }| {
        let buffer: &mut [[f32; CHANNELS as usize]] =
            sample::slice::to_frame_slice_mut(buffer).unwrap();
        sample::slice::equilibrium(buffer);

        // If the sampler is not currently active, play a note.
        if !sampler.is_active() {
            let vel = 0.3;
            sampler.note_on(pitch::LetterOctave(pitch::Letter::C, 4).to_hz(), vel);
            sampler.note_on(pitch::LetterOctave(pitch::Letter::E, 4).to_hz(), vel);
            sampler.note_on(pitch::LetterOctave(pitch::Letter::G, 1).to_hz(), vel);
        }

        sampler.fill_slice(buffer, SAMPLE_RATE);

        pa::Continue
    };

    let mut stream = pa.open_non_blocking_stream(settings, callback)?;
    stream.start()?;
    // Audio initialisation is complete. Start processing input.

    loop {
        let readline = rl.readline("♪♪♪ ");
        match readline {
            Ok(line) => {
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
