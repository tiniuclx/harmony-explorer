use crate::database::*;
use crate::music_theory::*;
use crate::schema::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub fn generate_chords() -> Vec<ChordNote> {
    use degrees::*;
    use intervals::*;
    vec![
        // Triads
        ("major", III, Maj3rd),
        ("major", V, Per5th),
        ("minor", III, Min3rd),
        ("minor", V, Per5th),
        ("diminished", III, Min3rd),
        ("diminished", V, Dim5th),
        ("augmented", III, Maj3rd),
        ("augmented", V, Aug5th),
        // Tetrads
        ("major seventh", III, Maj3rd),
        ("major seventh", V, Per5th),
        ("major seventh", VII, Maj7th),
        ("dominant seventh", III, Maj3rd),
        ("dominant seventh", V, Per5th),
        ("dominant seventh", VII, Min7th),
        ("minor seventh", III, Min3rd),
        ("minor seventh", V, Per5th),
        ("minor seventh", VII, Min7th),
        // Other 4-note chord
        ("major sixth", III, Maj3rd),
        ("major sixth", V, Per5th),
        ("major sixth", VI, Maj6th),
    ]
    .into_iter()
    .map(|t| ChordNote::note(t.0, t.1, t.2))
    .collect()
}

pub fn populate_database(db: &SqliteConnection) {
    diesel::insert_into(notes::table)
        .values(generate_chords())
        .execute(db)
        .unwrap();
}
