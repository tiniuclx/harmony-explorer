use crate::database::*;
use crate::music_theory::*;
use crate::schema::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub fn generate_chords() -> Vec<ChordNote> {
    use degree_intervals::*;
    vec![
        // Triads
        ("major", Maj3rd),
        ("major", Per5th),
        ("minor", Min3rd),
        ("minor", Per5th),
        ("diminished", Min3rd),
        ("diminished", Dim5th),
        ("augmented", Maj3rd),
        ("augmented", Aug5th),
        // Tetrads
        ("major seventh", Maj3rd),
        ("major seventh", Per5th),
        ("major seventh", Maj7th),
        ("dominant seventh", Maj3rd),
        ("dominant seventh", Per5th),
        ("dominant seventh", Min7th),
        ("minor seventh", Min3rd),
        ("minor seventh", Per5th),
        ("minor seventh", Min7th),
        ("major sixth", Maj3rd),
        ("major sixth", Per5th),
        ("major sixth", Maj6th),
        ("diminished seventh", Min3rd),
        ("diminished seventh", Dim5th),
        ("diminished seventh", Dim7th),
    ]
    .into_iter()
    .map(|t| ChordNote::note(t.0, (t.1).0, (t.1).1))
    .collect()
}

pub fn generate_names() -> Vec<ChordName> {
    vec![
        ("major", ""),
        ("major", "maj"),
        ("minor", "m"),
        ("minor", "-"),
        ("diminished", "dim"),
        ("diminished", "o"),
        ("diminished", "*"),
        ("augmented", "+"),
        ("augmented", "aug"),
        ("major seventh", "M7"),
        ("major seventh", "maj7"),
        ("dominant seventh", "7"),
        ("minor seventh", "m7"),
        ("minor seventh", "-7"),
        ("major sixth", "6"),
        ("major sixth", "maj6"),
        ("diminished seventh", "dim7"),
        ("diminished seventh", "o7"),
    ]
    .into_iter()
    .map(|t| ChordName::name(t.0, t.1))
    .collect()
}

pub fn populate_database(db: &SqliteConnection) {
    diesel::insert_into(notes::table)
        .values(generate_chords())
        .execute(db)
        .unwrap();

    diesel::insert_into(names::table)
        .values(generate_names())
        .execute(db)
        .unwrap();
}
