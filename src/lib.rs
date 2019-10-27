#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate enum_primitive_derive;
extern crate find_folder; // For easily finding the assets folder.
extern crate gag;
extern crate nom;
extern crate num_traits;
extern crate pitch_calc; // To work with musical notes.
extern crate portaudio; // For audio I/O
extern crate rustyline;
extern crate sample; // To convert portaudio sample buffers to frames.
extern crate sampler;

pub mod chord_library;
pub mod database;
pub mod music_theory;
pub mod parser;
pub mod schema;
